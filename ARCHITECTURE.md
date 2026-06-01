# Architecture

The architecture focuses on hardware-software decoupling, strict memory safety, thread safety, and platform-agnostic simulation. The system is split into four distinct tiers.

## 1. Core Protocol Layer (`spacecan`)
The `spacecan` crate is a pure, `#![no_std]` library. It knows nothing about WebSockets, UDP, or specific STM32 hardware. 
- **Primitives**: Defines `CanFrame` (8 bytes max) and `SpaceCANPacket` (up to 1024 bytes).
- **Assembler**: The `PacketAssembler` breaks large packets into 8-byte chunks, inserting 2-bit sequence flags (`01` First, `00` Middle, `10` Last, `11` Unsegmented) to reconstruct payloads across the bus.
- **Services**: The `ServiceManager` contains routing logic that inspects a packet's `packet_type` (e.g., 3 for ST03 Housekeeping) and dispatches it to the corresponding service handler.
- **Memory**: Backed entirely by `heapless::Vec` and `heapless::FnvIndexMap`. Zero dynamic allocation.

## 2. Firmware Layer (`spacecan-firmware`)
This crate is the bare-metal deployment of the core library.
- Targets `thumbv7em-none-eabihf` (STM32F4).
- Uses `cortex-m-rtic` to manage real-time concurrency.
- Initializes an Independent Watchdog (`IWDG`) to prevent silent processor lockups.
- Periodically dispatches heartbeat packets onto the physical CAN bus peripheral.

## 3. Virtual Network Layer (`spacecan-virtual`)
This crate provides host-side simulation without requiring physical hardware.
- It instantiates multiple independent processes (`controller` and `responder`).
- Replaces the physical CAN bus with a UDP Multicast network (bound to `224.0.0.123:5000`).
- The virtual nodes deserialize UDP datagrams, reconstruct them into `CanFrame` structs, and feed them into the exact same `spacecan` core library that the firmware uses.

## 4. Ground Station Dashboard (`dashboard` + `dashboard_server`)
Provides real-time visualization of the network.
- **`dashboard_server`**: A Rust Axum application that listens on the UDP Multicast network, converts the binary `CanFrame` data into JSON, and broadcasts it over WebSockets.
- **`dashboard` (React/Vite)**: Connects to the WebSocket, renders a scrolling live telemetry stream, and provides interactive buttons to inject PUS Telecommands back into the network.
