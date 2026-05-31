# ADR 0001: OS Portability and SocketCAN Decoupling

## Status
Approved

## Context
The initial `spacecan-virtual` simulation tool had a hard workspace-wide dependency on `socketcan`. Because the `socketcan` crate relies on Linux-only netlink interfaces (`neli` Netlink socket bindings and POSIX socket APIs), compiling the cargo workspace on Windows development systems failed immediately. This blocked Windows-based developer checkouts and CI platforms.

## Decision
1. **Move dependency to target blocks**: Restructure `spacecan-virtual/Cargo.toml` so that the `socketcan` dependency is gated under target-specific blocks (`[target.'cfg(target_os = "linux")'.dependencies]`).
2. **Conditional Compilation Guards**: Refactor the imports and async handlers in `controller.rs` and `responder.rs` using `#[cfg(target_os = "linux")]` compile gates, stubbing out SocketCAN functionality on Windows with warning outputs.
3. **Gate Build Link Directives**: Modify `spacecan-virtual/build.rs` to only link the standard `c` library (`cargo:rustc-link-lib=c`) on non-Windows platforms.

## Consequences
- **Positive**: The workspace compiles cleanly on Windows development environments, removing development blocks.
- **Positive**: Prepares the virtualization layer for a platform-independent UDP multicast network in Phase 5.
- **Negative**: The virtual simulation commands (`run-virtual`) do not execute CAN frames on Windows yet; this will be resolved in Phase 5 when we implement the virtual UDP transport.
