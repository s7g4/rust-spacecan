# SpaceCAN Examples

Usage examples for the SpaceCAN protocol library.

## Structure

```
examples/
├── basic/
│   ├── send_can.rs             # Construct and send a CAN frame
│   ├── recieve_can.rs          # Receive and decode a CAN frame
│   ├── sync_example.rs         # Synchronization message handling
│   └── heartbeat_example.rs    # Heartbeat monitoring loop
├── packet/
│   ├── split_packet.rs         # Fragment a large payload into CAN frames
│   ├── reassemble_packet.rs    # Reassemble fragments into a complete packet
│   └── full_packet_demo.rs     # End-to-end fragmentation and reassembly
├── services/
│   ├── packet_service.rs       # Service handler for packet processing
│   ├── service_full_demo.rs    # Full PUS service routing demonstration
│   └── service_splitter.rs     # Service-level packet splitting
└── vcan.sh                     # Script to set up a virtual CAN interface
```

## Running an Example

```bash
cargo run -p spacecan --example split_packet
```

Replace `split_packet` with the name of any example file (without the `.rs` extension).

## Prerequisites

- Rust stable toolchain
- The `spacecan` library compiled with `std` feature for host execution:
  ```bash
  cargo build -p spacecan --features std
  ```

## License

MIT License