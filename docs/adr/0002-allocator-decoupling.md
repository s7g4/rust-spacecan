# ADR 0002: Decoupling Global Allocator from Library Crate

## Status
Approved

## Context
The core `spacecan` library crate unconditionally registered a static `LockedHeap` global allocator inside `lib.rs` and initialized a tiny 8KB heap buffer. Because Rust allows only one global allocator to be registered in any compiled binary, this registry collided with standard test runners and desktop binaries linking `spacecan`. Standard tests ran out of memory (allocating metadata, test capturing, logs) or failed to compile with duplicate allocator symbol errors.

## Decision
1. **Remove Library Allocator Registry**: Delete the `#[global_allocator]` static allocator declaration and `init_allocator()` function from the library crate.
2. **Binary-Level Allocation**: Delegate all allocator registrations to binary and firmware targets. 
3. **Remove Redundant Binary Target**: Remove the redundant embedded `main.rs` binary configuration from the library crate to enforce a clear library boundary.

## Consequences
- **Positive**: Standard tests compile and run successfully using the host system allocator (UCrt/Glibc).
- **Positive**: The library becomes allocator-neutral and can be safely linked into std or custom no_std targets without namespace collision.
- **Positive**: Enforces clear design boundaries between libraries and firmware binaries.
