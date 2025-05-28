# spacecan-virtual

## Overview
`spacecan-virtual` is a virtual implementation of the SpaceCAN protocol stack designed for desktop environments. It uses async features and tokio runtime for asynchronous operations.

## Prerequisites
- Rust toolchain installed (preferably nightly for latest features)
- Cargo package manager
- Virtual CAN interface (e.g., `vcan0`) set up on your system for testing CAN frames

## Setting up Virtual CAN Interface

Before running the virtual CAN programs, you need to set up the virtual CAN interface on your Linux system.

Run the following commands with root privileges:

```bash
sudo modprobe vcan
sudo ip link add dev vcan0 type vcan
sudo ip link set up vcan0
```

These commands load the virtual CAN kernel module and create and activate the `vcan0` interface.

## Building

To build the `spacecan-virtual` package with async features enabled, run:

```bash
cargo build --release --features async --manifest-path=rust-spacecan/spacecan-virtual/Cargo.toml
```

## Running

After building, you can run the example responder or controller binaries:

```bash
cargo run --release --features async --bin responder --manifest-path=rust-spacecan/spacecan-virtual/Cargo.toml
```

```bash
cargo run --release --features async --bin controller --manifest-path=rust-spacecan/spacecan-virtual/Cargo.toml
```

Make sure your virtual CAN interface (e.g., `vcan0`) is up and running before starting these programs.

## Testing

You can test the virtual CAN communication by running both responder and controller simultaneously in separate terminals.

## Notes

- The async feature gates tokio and tokio-stream dependencies.
- The virtual CAN interface is required for proper operation.
- Logs and debug information will be printed to the console.

## Troubleshooting

- If you encounter build errors related to features, ensure you have enabled the `async` feature.
- For CAN interface issues, verify the virtual CAN device is created and active using the commands above.

## License

MIT License
