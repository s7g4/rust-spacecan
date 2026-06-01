use serde::{Deserialize, Serialize};
use socket2::{Domain, Protocol, Socket, Type};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use tokio::net::UdpSocket;

pub const MULTICAST_ADDR: &str = "224.0.0.123";
pub const PORT: u16 = 5000;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UdpCanFrame {
    pub id: u32,
    pub data: Vec<u8>,
}

pub fn create_multicast_socket() -> anyhow::Result<UdpSocket> {
    let multi_addr = MULTICAST_ADDR.parse::<Ipv4Addr>()?;
    let any_addr = Ipv4Addr::new(0, 0, 0, 0);

    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;

    // Allow multiple nodes to bind to the same port on the same machine
    socket.set_reuse_address(true)?;
    #[cfg(not(windows))]
    socket.set_reuse_port(true)?;

    // Bind
    let addr = SocketAddrV4::new(any_addr, PORT);
    socket.bind(&addr.into())?;

    // Join multicast group
    socket.join_multicast_v4(&multi_addr, &any_addr)?;

    // Enable loopback for local testing
    socket.set_multicast_loop_v4(true)?;

    // Convert to tokio UdpSocket
    socket.set_nonblocking(true)?;
    let std_socket: std::net::UdpSocket = socket.into();
    let tokio_socket = UdpSocket::from_std(std_socket)?;

    Ok(tokio_socket)
}

pub async fn send_multicast(socket: &UdpSocket, frame: &UdpCanFrame) -> anyhow::Result<()> {
    let data = serde_json::to_vec(frame)?;
    let target: SocketAddr = format!("{}:{}", MULTICAST_ADDR, PORT).parse()?;
    socket.send_to(&data, target).await?;
    Ok(())
}
