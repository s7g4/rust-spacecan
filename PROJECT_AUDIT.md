# Project Audit: rust-spacecan

- **`spacecan` (Core Library Crate)**:
  - **`primitives/`**: Data models for `CanFrame`, `HeartbeatData`, `SyncData`, `NodeInfo`, and `Packet` (fragmentation), plus a timer-helper.
  - **`services/`**: Incomplete implementations of ECSS-E-ST-70-41C (Packet Utilization Standard - PUS) services:
    - `ST01`: Request Verification (verification stages and reporting).
    - `ST03`: Housekeeping (defining and generating parameter reports).
    - `ST08`: Function Management (executing device functions by ID).
    - `ST17`: Test (simple connection/ping tests).
    - `ST20`: Parameter Management (reading and writing device telemetry parameters).
    - `core.rs`: A generic `ServiceManager` and `ServiceHandler` trait to register and route frames to services.
  - **`transport/`**:
    - `base.rs`: Defines the `Bus` trait and a thread-unsafe `BusImpl` backing storage.
    - `frame_buffer.rs`: A fixed-size double-ended queue (`VecDeque`) frame buffer.
    - `mock.rs`: A loopback mock transport for testing.
  - **`tests/`**: Basic unit and integration tests verifying primitive logic.

- **`spacecan-firmware` (Embedded Crate)**:
  - Targeting STM32G0 microcontroller (`stm32g0xx-hal`), but includes configurations for STM32F767ZI in `memory.x` and a custom Cortex-M7 target JSON.
  - A mock-based loopback example execution inside `src/lib.rs` configured with `#[entry]`.

- **`spacecan-virtual` (Simulator Crate)**:
  - Two tokio-based command-line binaries (`controller` and `responder`) that interface with the Linux SocketCAN socket on `vcan0` to demonstrate the protocol and PUS services.

## 2. What is Broken & Security Risks

### A. Windows Compilation Failure (SocketCAN Dependency)
The virtual testing environment (`spacecan-virtual`) has a hard dependency on `socketcan = "2.1.0"`. Because the `socketcan` crate uses Linux-specific APIs (`neli` Netlink socket bindings and POSIX socket operations), compiling the workspace on Windows fails immediately.
- **Root Cause**: Hard dependency on POSIX-exclusive bindings in a workspace-wide check.
- **Impact**: Cross-platform development is blocked; cannot run or test simulations on Windows.

### B. Global Allocator Leak in Library Crate
In `spacecan/src/lib.rs`, `#[global_allocator]` is defined unconditionally using the `LockedHeap` from the `linked_list_allocator` crate:
```rust
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();
```
- **Root Cause**: Defining a global allocator in a library crate. In Rust, only one global allocator can be registered in the final linked binary. If a binary pulls in `spacecan`, it is forced to use this 8KB static allocator.
- **Impact**: Any `std` binary (like `spacecan-virtual` or tests) linking `spacecan` will compile with this 8KB heap. This causes instant out-of-memory panics or memory corruption when executing standard allocations.

### C. Packet Reassembly Logic Bug
In `spacecan/src/protocol.rs`, the packet reassembly inside `receive_frame` is logic-broken:
```rust
pub fn receive_frame(&mut self) -> Result<Option<SpaceCANFrame>, SpaceCANError> {
    if let Some(can_frame) = self.transport.get_frame() {
        if let Some(packet) = self.packet_assembler.process_frame(can_frame.clone()) {
            // Reconstruct SpaceCANFrame from complete packet...
            return Ok(Some(frame));
        } else {
            // Single frame, not fragmented
            let frame = SpaceCANFrame::from_can_frame(can_frame)?;
            return Ok(Some(frame));
        }
    }
    Ok(None)
}
```
- **Root Cause**: If a frame is a fragment of a multi-frame packet, `process_frame` returns `None` until *all* fragments are received. However, when it returns `None`, the `else` block executes immediately, treating the incomplete fragment as a single, unfragmented frame.
- **Impact**: Fragments are processed immediately as independent frames. Their header bytes (representing total frames and index) are parsed as `service_type` and `subservice_type`, corrupting the data stream and crashing the service router.

### D. Thread Safety Violations (Undefined Behavior)
`BusImpl` (in `spacecan/src/transport/base.rs`) uses `UnsafeCell<Vec<CanFrame>>` to store frames:
```rust
pub struct BusImpl {
    buffer: UnsafeCell<Vec<CanFrame>>
}
```
In `not(feature = "embedded")` configurations (such as tests and `spacecan-virtual`), the synchronization wrapper `interrupt_free` compiles down to a direct function call:
```rust
fn interrupt_free<F, R>(f: F) -> R { f() }
```
- **Root Cause**: Reading and writing `UnsafeCell` concurrently across multiple threads without an active mutex or atomic lock.
- **Impact**: Calling `send` and `get_frame` concurrently from asynchronous tokio tasks in the simulation binaries triggers race conditions and compiler Undefined Behavior (UB), leading to data races and random memory segmentation faults.

### E. Microcontroller and Memory Configuration Mismatch
- **Crate Hardware target**: `spacecan-firmware`'s `Cargo.toml` currently references `stm32g0xx-hal` (Cortex-M0+ MCU).
- **Target JSON**: The workspace contains `thumbv7em-none-eabihf.json` configured for a Cortex-M7 core (`"cpu": "cortex-m7"`).
- **Memory.x**: Configured for `STM32F767ZI` (2MB FLASH, 512KB RAM).
- **Design Decision**: The hardware target will be aligned on **STM32F4** (Cortex-M4, e.g. STM32F407 on STM32F4Discovery board, native `thumbv7em-none-eabihf` target) as specified in the original readme and simulation specs.
- **Impact**: The HAL must be replaced with `stm32f4xx-hal` and `memory.x` updated to fit the STM32F4 memory layout. The Cortex-M7 target JSON should either be modified for Cortex-M4 or replaced with standard target flags.

### F. Missing Simulation Files
The root `README.md` documents a `renode/` directory with script files like `spacecan.resc` and `stm32f4_discovery.repl`. No such directory exists in the workspace.

## 3. What Should Be Removed

- **Orphan Crate Files**:
  - `spacecan/src/reciever.rs`: Redefines structs and traits, utilizes `std` filesystems, and is not in the compilation module tree.
  - `spacecan/src/controller.rs`: Incompatible duplicate implementation of network controllers.
  - `spacecan/src/parser.rs`: Redundant wrapping of CAN frame encoding.
- **Unconditional Library Allocator**:
  - The static `#[global_allocator]` setup inside `spacecan/src/lib.rs`.

## 4. What Should Be Rewritten

- **Transport Safety Wrapper**:
  - Replace the `UnsafeCell`-based loopback with thread-safe types (`spin::Mutex` or cross-platform primitives) for `no_std`, and standard `std::sync::Mutex` or channels for `std`.
- **Platform-Agnostic Virtual Transport**:
  - Rewrite `spacecan-virtual`'s transport layer to support cross-platform mock loops or UDP/TCP socket-based CAN framing so that simulations can run on Windows, macOS, and Linux without raw SocketCAN system calls.
- **Packet Assembler Router**:
  - Redesign `receive_frame` so that incomplete fragments are safely buffered and ignored by the service router until the assembly is complete.
- **Firmware Crate Integration**:
  - Re-align the firmware target, MCU HAL (e.g. choose standard STM32F4 or STM32G0), and `memory.x` to represent a singular, valid hardware platform.

## 5. What Should Be Preserved

- **PUS Service Structures**:
  - The core implementation logic of ST01, ST03, ST08, ST17, and ST20. They model the ECSS PUS protocols correctly and just require minor routing fixes.
- **Core Primitive Serialization**:
  - Heartbeat, sync, and frame serialization byte formats (`to_bytes`/`from_bytes`) are sound.
