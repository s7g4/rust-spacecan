# Rust SpaceCAN

## Overview

Rust SpaceCAN is a Rust workspace project implementing a CAN (Controller Area Network) protocol stack and firmware for embedded systems. It consists of two main crates:

- `spacecan`: A no_std Rust library providing CAN frame encoding, decoding, and protocol services.
- `spacecan-firmware`: Minimal firmware targeting STM32F4Discovery hardware implementation.
- `spacecan-virtual`: Virtual implementation of SpaceCAN for testing and simulation.

The project aims to provide a robust, embedded-friendly CAN protocol implementation with simulation support.

## 📌 Project Structure

```
rust-spacecan/
|   ├──spacecan/
│   |   ├── src/
│   |   |   ├── primitives/       # Core communication components
│   │   |   |   ├── can_frame.rs        # CAN Frame struct & serialization
│   │   |   |   ├── heartbeat.rs        # Heartbeat signal processing
│   │   |   |   ├── network.rs          # CAN network handling
│   │   |   |   ├── packet.rs           # Packet fragmentation & reassembly
│   │   |   |   ├── sync.rs             # Sync message handling
│   │   |   |   ├── timer.rs            # Periodic task scheduling
│   |   |   ├── services/        # Service layer components
│   │   |   |   ├── core.rs              # Packet routing & processing
│   │   |   |   ├── ST01_request_verification.rs  # Request verification
│   │   |   |   ├── ST03_housekeeping.rs          # Housekeeping service
│   │   |   |   ├── ST08_function_management.rs   # Function management
│   │   |   |   ├── ST17_test.rs                  # System test service
│   │   |   |   ├── ST20_parameter_management.rs  # Parameter management
│   |   |   ├── transport/       # Low-level transport layer
│   │   |   |   ├── base.rs              # Bus implementation
│   │   |   |   ├── frame_buffer.rs       # Frame buffering
│   │   |   |   ├── mock.rs         # Mock implementation
│   |   |   ├── tests/           # Unit test suite
│   |   |   |   ├── test_base.rs         # Unit tests for Bus implementation
│   |   |   |   ├── test_can_frame.rs    # Unit tests for CAN frames
│   |   |   |   ├── test_core.rs         # Unit tests for packet processing
│   |   |   |   ├── test_heartbeat.rs    # Unit tests for heartbeat service
│   |   |   |   ├── test_packet.rs       # Unit tests for packet fragmentation
│   |   |   |   ├── test_sync.rs         # Unit tests for sync processing
│   |   |   |   ├── test_timer.rs        # Unit tests for timer module
│   |   ├── examples/                     # Example implementations
│   |   |   ├── basic/
|   |   |   |   ├── heartbeat_example.rs    # Basic heartbeat example
|   |   |   |   ├── receive_can.rs          # Receive CAN example
|   |   |   |   ├── send_can.rs             # Send CAN example
|   |   |   |   ├── sync_example.rs         # Sync example
|   |   |   ├── packet/
|   |   |   |   ├── full_packet_demo.rs    # Full packet demo
|   |   |   |   ├── reassemblepacket.rs    # Reassemble packet example
|   |   |   |   ├── split_packet.rs    # Split packet example
|   |   |   ├── services/
|   |   |   |   ├── packet_service.rs   # packet service example
|   |   |   |   ├── service_full_demo.rs    #servive full demo
|   |   |   |   ├── service_splitter.rs    #service splitter example
|   |   |   ├── README.md   #documentation for examples
|   |   |   └── vcan.sh   #vcan documentation
|   |   ├── docs/
|   |   |   ├──ECSS-E-ST-50-15C.pdf    #ECSS standard defining CAN application layer.
|   |   |   ├──ECSS-E-ST-70-41C(15April2016).pdf    #ECSS Telecommand/Telemetry protocol document.
|   |   ├── Cargo.toml    # Rust package configuration
|   |   ├── Cargo.lock    #Lock file for dependency versions
|   ├──spacecan-firmware/
|   |   ├── src/
|   |   |   ├── main.rs    #Main entry point for firmware
|   |   |   ├── lib.rs     #Library part of firmware crate
|   |   ├──examples/
|   |   |   ├── firmware.rs    #Firmware example
|   |   |   ├──release  #release file
|   |   ├──Cargo.lock   #Lock file for dependency versions
|   |   ├──Cargo.toml   # Rust package configuration
|   |   ├──memory.x     #Memory configuration file
|   |   ├──thumbv7em-none-eabihf.json    #Target configuration file
|   ├──spacecan-virtual/
|   |   ├── src/
|   |   |   ├── main.rs    #Main entry point for virtual implementation
|   |   ├── Cargo.toml    # Rust package configuration
|   |   ├── README.md    #Project Documentation
|   |   ├── controller.rs    #Controller implementation
|   |   ├── responder.rs     #Responder implementation
|   ├── README.md               # Project documentation
|   ├── CONTRIBUTING.md         # Contribution guidelines
|   ├── LICENSE.md              # Project license
|   ├── Cargo.toml              # Rust package configuration
|   ├── Cargo.lock              #Lock file for dependency versions

```

## Workspace Structure

- `spacecan/`: Core CAN protocol library and examples.
- `spacecan-firmware/`: Firmware implementation for STM32F4Discovery.
- `spacecan-virtual/`: Virtual implementation for testing and simulation.

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

## Running Specific Implementations

To avoid interchanging the virtual and firmware implementations, always explicitly build and run the desired crate.

### Running spacecan-virtual

#### Prerequisites

- Rust toolchain installed (preferably nightly for latest features)
- Cargo package manager
- Virtual CAN interface (e.g., `vcan0`) set up on your Linux system

To set up the virtual CAN interface, run the following commands with root privileges:

```bash
sudo modprobe vcan
sudo ip link add dev vcan0 type vcan
sudo ip link set up vcan0
```

#### Build and Run

Build the package with async features enabled:

```bash
cargo build --release --features async -p spacecan-virtual
```

Run the example responder or controller binaries:

```bash
cargo run --release --features async --bin responder -p spacecan-virtual
```

```bash
cargo run --release --features async --bin controller -p spacecan-virtual
```

---

### Running spacecan-firmware

#### Prerequisites

- Rust installed with the appropriate target for embedded ARM Cortex-M:

```bash
rustup target add thumbv7em-none-eabihf
```

#### Build and Run

Build the firmware:

```bash
cargo build --release -p spacecan-firmware
```

Run the firmware:

```bash
cargo run -p spacecan-firmware
```

Or using the Cargo alias:

```bash
cargo run-firmware
```




## Running the Mock Example in `spacecan`

The `spacecan` crate includes a mock transport example demonstrating CAN frame encoding and decoding.

To run the example (requires std feature enabled):

```bash
cargo run --example heartbeat_example --features std
```

This example encodes a heartbeat CAN frame, sends it via a mock transport, and decodes it back.

## Running the Firmware in Renode Simulation

The firmware is designed to run on an STM32F4Discovery board.

## Dependencies and Features

- `spacecan` is a no_std crate by default, with optional `std` feature for examples and testing.
- Uses embedded Rust crates such as `cortex-m`, `embedded-hal`, `stm32f7xx-hal`, and `bxcan`.
- `spacecan-firmware` depends on `spacecan` and hardware abstraction layers for STM32F7 series.
- `spacecan-virtual` depends on `spacecan` and provides a virtual implementation for testing.

## Additional Notes

- The project uses a linked list allocator for dynamic memory in no_std environments.
- Panic handling is minimal and designed for embedded constraints.
- The Renode scripts define the hardware peripherals and memory layout for simulation.

## Contributing

Contributions and improvements are welcome. Please follow Rust embedded best practices.

For detailed contribution guidelines, please see the [CONTRIBUTING.md](CONTRIBUTING.md) file.

## License

This project is licensed under the terms specified in the [LICENSE.md](LICENSE.md) file.
[text](thumbv7em-none-eabihf.json)
---

For detailed usage or developer guides, additional documentation can be added in the `docs/` directory.

---
