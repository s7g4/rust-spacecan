use axum::{
    Router,
    extract::State,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
};
use std::sync::Arc;
use tokio::sync::broadcast;
use tower_http::services::ServeDir;

#[path = "network.rs"]
mod network;

struct AppState {
    tx: broadcast::Sender<String>,
    udp_socket: Arc<tokio::net::UdpSocket>,
}

#[cfg(feature = "async")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let udp_socket = network::create_multicast_socket()?;
    let udp_socket = Arc::new(udp_socket);

    // Channel for broadcasting UDP -> WS
    let (tx, _rx) = broadcast::channel(100);

    let app_state = Arc::new(AppState {
        tx: tx.clone(),
        udp_socket: udp_socket.clone(),
    });

    // Spawn task to read UDP and broadcast to WS
    let udp_sock_clone = udp_socket.clone();
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        let mut buf = vec![0u8; 2048];
        loop {
            if let Ok((len, _addr)) = udp_sock_clone.recv_from(&mut buf).await
                && let Ok(frame) = serde_json::from_slice::<network::UdpCanFrame>(&buf[..len])
                && let Ok(json_str) = serde_json::to_string(&frame)
            {
                let _ = tx_clone.send(json_str);
            }
        }
    });

    let app = Router::new()
        .fallback_service(ServeDir::new("../dashboard/dist"))
        .route("/ws", get(ws_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Dashboard server running on http://localhost:3000");
    axum::serve(listener, app).await?;
    Ok(())
}

#[cfg(not(feature = "async"))]
fn main() {
    println!("Async feature disabled.");
}

#[cfg(feature = "async")]
async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

#[cfg(feature = "async")]
async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    let mut rx = state.tx.subscribe();

    loop {
        tokio::select! {
            Ok(msg) = rx.recv() => {
                if socket.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }
            Some(msg) = socket.recv() => {
                if let Ok(Message::Text(text)) = msg {
                    if let Ok(frame) = serde_json::from_str::<network::UdpCanFrame>(&text) {
                        let _ = network::send_multicast(&state.udp_socket, &frame).await;
                    }
                } else if msg.is_err() {
                    break;
                }
            }
        }
    }
}
