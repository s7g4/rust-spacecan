# Engineering Roadmap: rust-spacecan

## Phase 1: Workspace Sanitization & Cross-Platform Portability
- **Goal**: Resolve immediate compiler errors on Windows, clean up orphaned files, and set up cargo profiles.
- **Why it Exists**: Unlocks multi-developer collaboration and local development on Windows systems by removing platform-specific SocketCAN blockers.
- **Inputs**: Root workspace and crate configuration files.
- **Outputs**: Compiling workspace skeletons across Windows and Linux.
- **Dependencies**: None.
- **Risks**: None (mostly cleanup).
- **Success Criteria**: `cargo check --workspace` passes successfully on Windows and Linux development systems.
- **Metrics**: 0 compilation errors across test platforms.
- **Deliverables**: Updated `Cargo.toml` configurations; deletion of dead files (`reciever.rs`, `controller.rs`, `parser.rs`).
- **Expected Commits**:
  - `chore: remove unused/orphaned source files`
  - `build: configure platform-independent workspaces`
- **Expected Documentation**: Updated [README.md](README.md) workspace notes.

## Phase 2: Memory Integrity & Concurrency Safety
- **Goal**: Remove library-level global allocator leaks and resolve `UnsafeCell` undefined behavior.
- **Why it Exists**: Ensures thread safety in multi-threaded simulation contexts and allows normal standard allocations for simulation runtimes.
- **Inputs**: `spacecan/src/lib.rs` and `spacecan/src/transport/`.
- **Outputs**: Leak-free, thread-safe transport wrappers.
- **Dependencies**: Phase 1 completed.
- **Risks**: Modifying memory configurations could impact stack sizes in target MCUs.
- **Success Criteria**: No global allocators in library code; zero `UnsafeCell` instances; validation via cargo test.
- **Metrics**: 0 compiler warnings regarding Send/Sync traits; 100% test pass.
- **Deliverables**: Thread-safe `BusImpl` using safe Mutex bindings (with conditional compilation: `critical-section` on embedded, standard `Mutex` on desktop).
- **Expected Commits**:
  - `refactor: remove global allocator registry from library`
  - `fix: replace UnsafeCell loopback with safe thread synchronization`
- **Expected Documentation**: Write ADR-01 (Architecture Decision Record) on Memory Allocation Strategy.

## Phase 3: Protocol Routing & Packet Assembly Fixes
- **Goal**: Fix the fragmentation reassembly bug in `receive_frame`.
- **Why it Exists**: Resolves the bug where incomplete packet fragments are leaked as full frames, corrupting service routers.
- **Inputs**: `spacecan/src/protocol.rs` and `spacecan/src/primitives/packet.rs`.
- **Outputs**: A packet assembler that holds fragments and forwards only assembled frames.
- **Dependencies**: Phase 2 completed.
- **Risks**: Fragile packet logic might introduce latency if timeouts are poorly configured.
- **Success Criteria**: Integration test demonstrates multi-frame packets assembling and routing correctly, without fragment leakage.
- **Metrics**: 100% of fragmented test packets assemble successfully.
- **Deliverables**: Rewritten `receive_frame` logic; fragmentation unit tests.
- **Expected Commits**:
  - `fix: correct packet reassembly fragment leakage in receive_frame`
  - `test: add unit tests for multi-frame packet reassembly`
- **Expected Documentation**: Documentation of packet reassembly state machine in `ARCHITECTURE.md`.

## Phase 4: STM32F4 Hardware Integration & Target Realignment
- **Goal**: Transition firmware configuration from STM32G0 to STM32F4 series.
- **Why it Exists**: Aligns the codebase with a target MCU (Cortex-M4, STM32F407) that features native hardware CAN controllers (bxCAN).
- **Inputs**: `spacecan-firmware/` source files, `memory.x`, target JSON.
- **Outputs**: Compile-ready bare-metal binary targeting STM32F4.
- **Dependencies**: Phase 3 completed.
- **Risks**: bxCAN register setups may vary slightly across different boards.
- **Success Criteria**: Firmware compiles for target `thumbv7em-none-eabihf` and links successfully with `memory.x`.
- **Metrics**: Firmware binary size footprint fits within flash ranges.
- **Deliverables**: Reconfigured `memory.x` for STM32F4; updated `Cargo.toml` with `stm32f4xx-hal` and `bxcan` crate bindings.
- **Expected Commits**:
  - `refactor: migrate firmware HAL dependencies from STM32G0 to STM32F4`
  - `build: configure stm32f4 linker and memory layout`
- **Expected Documentation**: Write ADR-02 detailing Target Microcontroller Configuration.

## Phase 5: Virtual UDP Multi-Node Simulation Network
- **Goal**: Implement a platform-agnostic virtual transport using UDP sockets.
- **Why it Exists**: Replaces SocketCAN with a cross-platform emulation socket that runs on Windows and Linux without system-level dependencies.
- **Inputs**: `spacecan-virtual/` crate.
- **Outputs**: Interactive simulation binaries (`controller`, `responder`) working via UDP multicast.
- **Dependencies**: Phase 3 completed.
- **Risks**: Port mapping issues if ports are blocked by firewall rules on Windows.
- **Success Criteria**: Run controller and responder simultaneously on Windows, exchanging telemetry packets.
- **Metrics**: Frame loss rate < 1% over local UDP.
- **Deliverables**: UDP socket-based `Bus` driver; working controller and responder scripts.
- **Expected Commits**:
  - `feat: implement cross-platform UDP simulation bus`
  - `chore: adapt spacecan-virtual binaries to use UDP transport`
- **Expected Documentation**: User Guide on running local multi-node simulations.
