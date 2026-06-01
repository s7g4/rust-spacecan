# Engineering Roadmap: rust-spacecan

## Phase 1: Workspace Sanitization & Cross-Platform Portability ✓
- **Status**: Complete.
- **Goal**: Resolve compilation failures on Windows, remove orphaned files, configure workspace profiles.
- **Deliverables**: Platform-conditional SocketCAN dependency, removed dead source files (`reciever.rs`, `controller.rs`, `parser.rs`), cleaned `Cargo.toml` configurations.

## Phase 2: Memory Integrity & Concurrency Safety ✓
- **Status**: Complete.
- **Goal**: Remove the library-level global allocator and fix `UnsafeCell` undefined behavior.
- **Deliverables**: Thread-safe `BusImpl` using `std::sync::Mutex` on host and `cortex_m::interrupt::Mutex` on embedded. Global allocator removed from library crate.

## Phase 3: Protocol Routing & Packet Assembly Fixes ✓
- **Status**: Complete.
- **Goal**: Fix the fragmentation reassembly bug in `receive_frame`.
- **Deliverables**:
  - `ST_FRAGMENTED` (0xFF) sentinel service type to tag fragmented frames on the wire.
  - `Packet::split` chunk size reduced to 4 bytes to fit within 8-byte CAN payload (2 header + 2 service/subservice + 4 data).
  - `PacketAssembler::process_fragment` now accepts `&SpaceCANFrame`.
  - `send_packet` prepends real service/subservice to payload before fragmenting.
  - `receive_frame` checks `ST_FRAGMENTED` to route fragmented vs. single-frame messages.
  - Packet test suite rewritten and registered (4 tests passing).

## Phase 4: STM32G0 Hardware Integration
- **Status**: Pending.
- **Goal**: Implement bxCAN peripheral initialization and integrate `SpaceCANProtocol` with the hardware bus driver.
- **Inputs**: `spacecan-firmware/src/main.rs`, STM32G071 bxCAN registers.
- **Outputs**: Firmware that initializes CAN, registers RX/TX ISRs, and enters a protocol event loop.
- **Dependencies**: Phase 3 completed.
- **Risks**: bxCAN filter bank configuration varies across STM32G0 revisions.
- **Success Criteria**: Firmware compiles for `thumbv6m-none-eabi`, links with `memory.x`, and initializes the CAN peripheral on hardware.
- **Deliverables**: CAN initialization in `main.rs`, ISR handlers, hardware `Bus` trait implementation.

## Phase 5: Virtual UDP Multi-Node Simulation Network
- **Status**: Pending.
- **Goal**: Replace SocketCAN with a cross-platform UDP multicast transport.
- **Inputs**: `spacecan-virtual/` crate.
- **Outputs**: Controller and responder binaries communicating over UDP on Windows and Linux.
- **Dependencies**: Phase 3 completed.
- **Risks**: UDP port conflicts or firewall rules on Windows.
- **Success Criteria**: Both binaries exchange telecommand/telemetry packets over local UDP without SocketCAN.
- **Deliverables**: UDP socket-based `Bus` driver, updated controller and responder binaries.
