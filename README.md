# SpaceCAN

## Overview

SpaceCAN is a Rust workspace implementing a CAN (Controller Area Network) protocol stack for embedded spacecraft systems. It provides CAN frame encoding/decoding, ECSS PUS service routing, multi-frame packet fragmentation, and a virtual simulation harness.

The workspace contains three crates:

- **spacecan** — `no_std` core library with CAN primitives, protocol handling, and PUS services.
- **spacecan-firmware** — Bare-metal firmware targeting STM32G071 (Cortex-M0+).
- **spacecan-virtual** — Tokio-based simulation binaries for host-side protocol testing.

## Project Structure

```
rust-spacecan/
├── spacecan/                   # Core protocol library (no_std)
│   ├── src/
│   │   ├── primitives/         # CanFrame, Packet, Heartbeat, Sync, Timer
│   │   ├── services/           # ECSS PUS service handlers (ST01–ST20)
│   │   ├── transport/          # Bus trait, BusImpl, FrameBuffer, MockTransport
│   │   ├── tests/              # Unit and integration tests
│   │   ├── protocol.rs         # SpaceCANFrame, SpaceCANProtocol
│   │   └── lib.rs              # Crate root and constants
│   ├── examples/               # Usage examples (basic, packet, services)
│   └── Cargo.toml
├── spacecan-firmware/          # Bare-metal firmware crate
│   ├── src/
│   │   ├── main.rs             # Cortex-M entry point
│   │   └── lib.rs
│   ├── build.rs                # Copies memory.x to linker search path
│   ├── memory.x                # STM32G071RB linker memory layout
│   └── Cargo.toml
├── spacecan-virtual/           # Host simulation binaries
│   ├── controller.rs           # Virtual CAN controller node
│   ├── responder.rs            # Virtual CAN responder node
│   ├── src/main.rs
│   └── Cargo.toml
├── .github/workflows/ci.yml   # CI/CD pipeline
├── Cargo.toml                  # Workspace root
└── README.md
```

## Building

### Prerequisites

```bash
# Stable Rust toolchain
rustup toolchain install stable

# ARM target for firmware cross-compilation
rustup target add thumbv6m-none-eabi
```

### Library and Simulation Binaries

```bash
cargo build -p spacecan
cargo build -p spacecan-virtual
```

### Firmware (Cross-Compilation)

```bash
cargo build -p spacecan-firmware --target thumbv6m-none-eabi
```

## Running

### Virtual Simulation

Open two terminals and run:

```bash
cargo run -p spacecan-virtual --bin controller
```

```bash
cargo run -p spacecan-virtual --bin responder
```

### Firmware

Flash the compiled binary to an STM32G071 target via probe-rs, OpenOCD, or ST-Link.

## Testing

```bash
cargo test -p spacecan
```

## Features

The `spacecan` library supports the following feature flags:

| Feature    | Description                                       |
|------------|---------------------------------------------------|
| `std`      | Enables tokio, serde_json, anyhow for host builds |
| `embedded` | Enables cortex-m, cortex-m-rt, embedded-hal, nb   |
| `defmt`    | Enables defmt structured logging                  |

## CI/CD

The GitHub Actions pipeline runs on every push and pull request to `main`:

- **Formatting** — `cargo fmt --all --check`
- **Clippy** — Per-package lint checks with `-D warnings`
- **Tests** — `spacecan` and `spacecan-virtual` on Ubuntu and Windows
- **Embedded Check** — `cargo check -p spacecan --features embedded`
- **Firmware Build** — Cross-compilation to `thumbv6m-none-eabi`

## License

MIT License
