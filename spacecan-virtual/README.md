# spacecan-virtual

Host-side simulation of the SpaceCAN protocol stack using tokio async runtime.

## Binaries

- **controller** — Sends telecommand frames and fragmented packets to the responder node.
- **responder** — Listens for incoming frames, routes them through the PUS service manager, and returns telemetry responses.

## Prerequisites

- Stable Rust toolchain
- Linux: optional SocketCAN interface (`vcan0`) for raw CAN transport

### Setting Up Virtual CAN (Linux Only)

```bash
sudo modprobe vcan
sudo ip link add dev vcan0 type vcan
sudo ip link set up vcan0
```

## Building

From the workspace root:

```bash
cargo build -p spacecan-virtual
```

## Running

Open two terminals:

```bash
cargo run -p spacecan-virtual --bin controller
```

```bash
cargo run -p spacecan-virtual --bin responder
```

## Features

| Feature | Default | Description                          |
|---------|---------|--------------------------------------|
| `async` | yes     | Enables tokio and tokio-stream deps  |

## License

MIT License
