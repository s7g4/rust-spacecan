# Project Audit: rust-spacecan

## 1. Crate Overview

- **`spacecan` (Core Library)**:
  - **`primitives/`**: Data models for `CanFrame`, `Packet` (fragmentation/reassembly), `HeartbeatManager`, `SyncManager`, `NetworkManager`, and `Timer`.
  - **`services/`**: ECSS PUS service implementations:
    - `ST01`: Request Verification
    - `ST03`: Housekeeping
    - `ST08`: Function Management
    - `ST17`: Test
    - `ST20`: Parameter Management
    - `core.rs`: `ServiceManager` trait-based router.
  - **`transport/`**:
    - `base.rs`: `Bus` trait and `BusImpl` using platform-conditional `Mutex` (cortex-m interrupt-free on embedded, `std::sync::Mutex` on host).
    - `frame_buffer.rs`: Fixed-capacity `VecDeque` ring buffer.
    - `mock.rs`: Loopback mock transport for tests.
  - **`tests/`**: Unit tests for packet fragmentation/reassembly and integration tests for service routing.

- **`spacecan-firmware` (Embedded Crate)**:
  - Targets STM32G071 (Cortex-M0+, `thumbv6m-none-eabi`).
  - HAL: `stm32g0xx-hal` with `stm32g071` and `i2c-blocking` features.
  - Memory layout: 128K flash, 36K RAM (STM32G071RB).
  - Entry point is a minimal loop stub pending bxCAN integration.

- **`spacecan-virtual` (Simulator Crate)**:
  - Two tokio-based binaries (`controller`, `responder`) for host-side protocol simulation.
  - Conditionally depends on `socketcan` on Linux targets.

## 2. Resolved Issues

### A. Windows Compilation (Phase 1)
**Status**: Fixed. SocketCAN dependency is now gated behind `cfg(target_os = "linux")` in `spacecan-virtual/Cargo.toml`. The workspace compiles on Windows and Linux.

### B. Global Allocator Leak (Phase 1)
**Status**: Fixed. The `#[global_allocator]` and `linked_list_allocator` dependency have been removed from the library crate.

### C. Packet Reassembly Bug (Phase 3)
**Status**: Fixed. `receive_frame` now parses every incoming `CanFrame` into a `SpaceCANFrame` first and checks `service_type`. Frames tagged `ST_FRAGMENTED` (0xFF) are routed to `PacketAssembler::process_fragment`; all other frames are returned directly. The old code path that leaked incomplete fragments as full frames has been eliminated.

### D. Thread Safety (Phase 2)
**Status**: Fixed. `BusImpl` now uses `std::sync::Mutex` on host and `cortex_m::interrupt::Mutex<RefCell<_>>` on embedded. The `UnsafeCell` wrapper has been removed.

### E. MCU Configuration Mismatch (CI Fix)
**Status**: Fixed. `memory.x` now matches STM32G071RB (128K/36K). The CI target triple is `thumbv6m-none-eabi` (Cortex-M0+). A `build.rs` copies `memory.x` to the linker search path.

## 3. Open Items

### Phase 4: Hardware Integration
- bxCAN peripheral initialization in firmware `main.rs`.
- ISR registration for CAN RX/TX events.
- Integration of `SpaceCANProtocol` with the hardware bus driver.

### Phase 5: UDP Simulation Transport
- Replace SocketCAN dependency with a cross-platform UDP multicast transport.
- Enable controller/responder to run on Windows without system-level CAN drivers.

## 4. Preserved Components

- **PUS Service Structures**: ST01, ST03, ST08, ST17, ST20 implementations are correct and route frames through the `ServiceManager` trait.
- **Primitive Serialization**: Heartbeat, sync, and CAN frame `to_bytes`/`from_bytes` are sound.
- **Packet Fragmentation**: `Packet::split` correctly chunks data into 4-byte fragments with 2-byte headers (total_frames, index).
