# SpaceCAN

## Overview

SpaceCAN is a Rust workspace implementing a CAN (Controller Area Network) protocol stack for embedded spacecraft systems. It provides CAN frame encoding/decoding, ECSS PUS service routing, multi-frame packet fragmentation, and a virtual simulation harness.

The workspace is strictly partitioned into three distinct operational domains:

- **spacecan** — The `#![no_std]` core protocol library. It implements ECSS PUS services and transport protocols with a strict **zero-allocation** architecture using `heapless`.
- **spacecan-firmware** — Bare-metal firmware targeting STM32F4 (Cortex-M4F) utilizing the `cortex-m-rtic` realtime concurrency framework.
- **spacecan-virtual** — A Tokio-based host simulation environment featuring virtual nodes communicating over UDP Multicast, and an Axum WebSocket bridge.
- **dashboard** — A React/Vite Ground Station UI that interfaces with the virtual nodes.

## Building & Linting

Because the repository mixes standard OS targets and bare-metal ARM targets, you must strictly separate your build and linting commands.

### 1. Virtual Nodes & Core Library (Host Target)

```powershell
cargo check -p spacecan -p spacecan-virtual
cargo clippy -p spacecan -p spacecan-virtual -- -D warnings
cargo test -p spacecan -p spacecan-virtual
```

### 2. Firmware (STM32F4 ARM Target)

```powershell
cargo check -p spacecan-firmware --target thumbv7em-none-eabihf
cargo clippy -p spacecan-firmware --target thumbv7em-none-eabihf -- -D warnings
```

## Running the Project

To experience the full SpaceCAN simulation, you need to run the Virtual Nodes, the WebSocket Bridge, and the React Dashboard concurrently.

### 1. Start the Virtual Simulation Nodes
Open two separate terminals in the project root and run:
```powershell
cargo run -p spacecan-virtual --bin controller
```
```powershell
cargo run -p spacecan-virtual --bin responder
```
*(These nodes will immediately begin broadcasting heartbeat frames over UDP Multicast `224.0.0.123:5000`)*

### 2. Start the Dashboard Server (WebSocket Bridge)
Open a third terminal in the project root and run:
```powershell
cargo run -p spacecan-virtual --bin dashboard_server
```

### 3. Run the Ground Station Dashboard (Frontend)
Open a fourth terminal, navigate into the `dashboard` directory, and start the Vite UI:
```powershell
cd dashboard
npm install
npm run dev
```
Finally, open your browser to **http://localhost:5173**. You will see the telemetry stream parsing real-time UDP frames, and you can inject Telecommands via the UI.

*(Alternatively, you can run `npm run build` inside the `dashboard` folder, and the Rust backend will automatically serve the UI on `http://localhost:3000`)*.

## CI/CD Pipeline

The GitHub Actions pipeline (`ci.yml`) runs concurrently on every push and pull request. It is strictly split into two isolation jobs to prevent feature-leakage:
- **Host CI**: Lints and tests the `spacecan` library and `spacecan-virtual` binaries on Windows using standard targets.
- **Firmware CI**: Validates the embedded `#![no_std]` constraints by cross-compiling and linting `spacecan-firmware` to `thumbv7em-none-eabihf`.
