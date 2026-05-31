# DevLog: rust-spacecan Development

## Day 1: 2026-05-31

### Goal
Execute **Phase 1: Workspace Sanitization & OS Portability** to ensure the entire workspace compiles and tests run successfully on Windows host development machines.

### Work Completed
1. Deleted orphaned/unused code files in `spacecan/src/` (`reciever.rs`, `controller.rs`, `parser.rs`) that clutter the module system.
2. Deleted redundant example target `examples/firmware.rs` from `spacecan-firmware`.
3. Moved the Linux-only `socketcan` dependency in `spacecan-virtual` to target-specific dependencies.
4. Gate SocketCAN imports and main logic in `spacecan-virtual`'s `controller.rs` and `responder.rs` using `#[cfg(target_os = "linux")]`.
5. Removed redundant global allocator logic and `init_allocator` from the core library `lib.rs` and deleted the redundant duplicate binary target `main.rs` inside the core `spacecan` crate.
6. Guarded `defmt` format implementations in `spacecan/src/protocol.rs` behind `#[cfg(all(feature = "defmt", not(test)))]` to prevent host tests from linking target logging stubs.
7. Implemented a host unit testing link-stub module (`defmt_mock.rs`) inside `spacecan/src/tests/` to satisfy MSVC/GCC linkers on developer desktops.
8. Gated the `libc` linking build script directive in `spacecan-virtual/build.rs` to ignore Windows hosts.
9. Reconfigured `spacecan-firmware/Cargo.toml` to set `test = false` on firmware binaries, libraries, and examples.
10. Validated host-side execution via `cargo test --workspace`.

### Problems Encountered & Root Cause Analysis

#### Problem A: Linux SocketCAN & `neli` Compilation Failures on Windows
- **Symptom**: Compiling `spacecan-virtual` on Windows failed due to netlink socket definitions (`sockaddr_nl` not found in `libc`).
- **Root Cause**: Windows does not natively support Linux socket netlink channels (`neli`) or SocketCAN APIs. `spacecan-virtual` declared a hard dependency on `socketcan` workspace-wide.

#### Problem B: Duplicate Panic Handler & Test Runner Gating
- **Symptom**: `cargo test --workspace` crashed on the embedded `spacecan-firmware` crate due to duplicate `panic_impl` symbol definitions (UCrt/std's panic system vs `panic-halt`).
- **Root Cause**: The Rust standard test runner automatically compiles binary and library targets in the workspace as host standard tests, linking `std` into a `#![no_std]` crate that uses `panic-halt`.

#### Problem C: MSVC Linker Failures on `c.lib`
- **Symptom**: Compiling `spacecan-virtual` on Windows host test runs failed with `LINK : fatal error LNK1181: cannot open input file 'c.lib'`.
- **Root Cause**: `spacecan-virtual`'s `build.rs` unconditionally emitted `cargo:rustc-link-lib=c`, instructing the linker to find `c.lib`, which does not exist on Microsoft MSVC toolchains (where C runtime is ucrt/msvcrt).

#### Problem D: Host-side Unit Test Linker Errors on `defmt`
- **Symptom**: Standard unit tests failed during linking on host because they were unable to resolve global symbols (e.g. `_defmt_acquire`, `_defmt_timestamp`, trace markers).
- **Root Cause**:
  1. The library crate unconditionally defined an 8KB `LockedHeap` global allocator inside `lib.rs` which starved test harnesses of memory.
  2. Cargo feature unification (under resolver = "2") enabled the `defmt` feature for `spacecan` library globally since `spacecan-firmware` requested it. When `defmt` was compiled on host, it generated references to external global logging symbols that were not provided by the test suite.

### Fixes Applied

1. **OS Gating**: Declared target-specific dependencies inside `Cargo.toml` targets (`[target.'cfg(target_os = "linux")'.dependencies]`).
2. **Build Gating**: Gated the build script lib directives to exclude Windows hosts (`#[cfg(not(target_os = "windows"))]`).
3. **Firmware Test Gating**: Configured `test = false` in `spacecan-firmware/Cargo.toml` for bin, lib, and examples targets to tell Cargo to skip them during host testing.
4. **Allocator Neutrality**: Deleted `#[global_allocator]` and `init_allocator` from the core library, deferring allocator registrations entirely to target firmware binaries (ensuring standard tests use standard allocators).
5. **Defmt Mock Symbols**: Created `defmt_mock.rs` containing target stub definitions (`#[unsafe(no_mangle)] extern "C"`) and registered it in `lib.rs` whenever `all(feature = "defmt", not(target_os = "none"))` is active.

### Lessons Learned
1. **Never Register Allocators in Libraries**: Library crates must be allocator-agnostic. Global allocator registrations must be handled exclusively by application/firmware crates.
2. **Cargo Feature Unification**: Feature states are shared across dependencies in a workspace during cargo test. We must use conditional logic like `#[cfg(all(feature = "x", not(test)))]` to prevent host tests from inheriting target-specific linker constraints.
3. **Target-Specific Build Steps**: Gating build scripts and target configurations (`test = false`) is essential when mixing bare-metal binaries and desktop simulation tools in a single Cargo workspace.

---

### Metrics
- **Deleted Files**: 4 (`spacecan/src/reciever.rs`, `spacecan/src/controller.rs`, `spacecan/src/parser.rs`, `spacecan-firmware/examples/firmware.rs`).
- **Workspace Build Check**: 100% successful on Windows host.
- **Test Executions**: 6 integration tests executed and passed successfully.
- **Compiler Failures**: 0.

---

### Next Steps
Proceed to **Phase 2: Allocator & Thread-Safety (UB) Refactoring** to replace target-unsafe internal mutability wrappers with thread-safe types, and restructure multi-threading layers.

---

## Day 2: 2026-05-31

### Goal
Execute **Phase 2: Memory Integrity & Concurrency Safety** to replace the unsafe `UnsafeCell` loopback and target-unsafe `RefCell` structures in `spacecan/src/transport/` with target-specific safe `Mutex` primitives.

### Work Completed
1. Refactored `BusImpl` in `spacecan/src/transport/base.rs` to replace `UnsafeCell` with `std::sync::Mutex` on host and `cortex_m::interrupt::Mutex` + `RefCell` on embedded.
2. Refactored `FrameBuffer` in `spacecan/src/transport/frame_buffer.rs` to replace the pseudo host lock `RefCell` with `std::sync::Mutex` on host targets.
3. Refactored `MockTransport` in `spacecan/src/transport/mock.rs` to use `std::sync::Mutex` on host.
4. Added `#[cfg(not(feature = "embedded"))] extern crate std;` to the core transport files to link std in `#![no_std]` crates on host systems.
5. Untracked target compilation directories and `.Rhistory` junk files from Git, updating `.gitignore` to ignore them recursively.
6. Validated compilation and test execution on Windows host.

### Problems Encountered & Root Cause Analysis

#### Problem A: Missing Crate std in `#![no_std]` Core Crates on Host
- **Symptom**: Compiling `spacecan` library on host tests failed with `error[E0433]: cannot find module or crate std in this scope`.
- **Root Cause**: Since `spacecan` core library is declared `#![no_std]`, the compiler does not automatically bring `std` into the namespace even when compiling for host environments where `std` is available.
- **Fix**: Added `#[cfg(not(feature = "embedded"))] extern crate std;` to the top of the transport module files to tell the compiler to link `std` on host compilation runs.

### Metrics
- **Refactored Files**: 3 (`spacecan/src/transport/base.rs`, `spacecan/src/transport/frame_buffer.rs`, `spacecan/src/transport/mock.rs`).
- **Workspace Build Check**: 100% successful with and without the `embedded` feature flag.
- **Test Executions**: 6 integration tests executed and passed successfully.
- **Compiler Failures**: 0.

---

### Next Steps
Proceed to **Phase 3: Protocol Routing & Packet Assembly Fixes** to resolve the fragment reassembly logic error in `receive_frame`.
