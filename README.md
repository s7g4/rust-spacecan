# SpaceCAN

## Overview

SpaceCAN is a Rust workspace project implementing a CAN (Controller Area Network) protocol stack and firmware for embedded systems. It consists of three main crates:

- `spacecan`: A no_std Rust library providing CAN frame encoding, decoding, and protocol services.
- `spacecan-firmware`: Minimal firmware targeting STM32F4 hardware.
- `spacecan-virtual`: Virtual implementation of SpaceCAN for testing and simulation on host systems.

## Project Structure

```
rust-spacecan/
├── spacecan/                # Core CAN protocol library
│   ├── src/
│   │   ├── primitives/      # Data models and base components
│   │   ├── services/        # ECSS PUS services implementation
│   │   ├── transport/       # Hardware abstraction and bus traits
│   │   ├── tests/           # Protocol test suite
│   ├── Cargo.toml
├── spacecan-firmware/       # Embedded target implementation
│   ├── src/
│   │   ├── main.rs          # Firmware entry point
│   ├── memory.x             # Linker memory configuration
│   ├── Cargo.toml
├── spacecan-virtual/        # Desktop simulation binaries
│   ├── src/
│   │   ├── controller.rs    # Virtual node controller
│   │   ├── responder.rs     # Virtual node responder
│   ├── Cargo.toml
├── Cargo.toml               # Workspace configuration
└── README.md
```

## Building the Project

Ensure you have Rust installed with the appropriate target for embedded ARM Cortex-M:

```bash
rustup target add thumbv7em-none-eabihf
```

### Build the entire workspace

From the root directory:

```bash
cargo build --release
```

### Build individual crates

- Build `spacecan` library:
```bash
cargo build --release -p spacecan
```

- Build `spacecan-firmware` firmware:
```bash
cargo build --release -p spacecan-firmware
```

- Build `spacecan-virtual` virtual implementation:
```bash
cargo build --release -p spacecan-virtual
```

## Running the Implementations

### Virtual Implementation (Host System)

```bash
cargo run -p spacecan-virtual --bin controller
cargo run -p spacecan-virtual --bin responder
```

Or using the Cargo aliases:
```bash
cargo run-virtual
```

### Firmware Implementation (Target Hardware)

```bash
cargo run -p spacecan-firmware
```

Or using the Cargo alias:
```bash
cargo run-firmware
```

## Dependencies and Target Information

- `spacecan` is a `no_std` crate by default, with an optional `std` feature for examples and testing.
- The `spacecan-firmware` crate depends on hardware abstraction layers for STM32F4 (`stm32f4xx-hal`) and targets the Cortex-M4 architecture (`thumbv7em-none-eabihf`).
- `spacecan-virtual` depends on `tokio` for host-side simulation and execution.

## License

MIT License
