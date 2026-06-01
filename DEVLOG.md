# Development Log

## Overview
This document logs the engineering journey, architectural mistakes made, refactoring decisions, and the lessons learned while building the SpaceCAN protocol stack.

## Mistakes and Refactoring

### The `extern crate alloc` Mistake
Initially, the `spacecan` library heavily utilized `std::vec::Vec` and `std::collections::BTreeMap`. When transitioning to a bare-metal embedded environment (STM32F4), I encountered massive linker errors because dynamic memory allocators are fundamentally unsafe and non-deterministic for realtime flight software.

I briefly attempted a "hack" by injecting a static allocator (`linked_list_allocator` of 32KB) into the firmware to force the `Vec` types to compile. This was an architectural mistake. It defeated the purpose of deterministic memory boundaries and violated MISRA-C/Rust guidelines for critical systems.

**The Solution:**
I conducted a full, deep refactoring to eradicate `alloc` entirely from the workspace. I imported the `heapless` crate and transitioned every dynamic vector and map to statically backed arrays (`heapless::Vec<u8, 1024>`, `heapless::FnvIndexMap`). This successfully dropped the zero-allocation constraint into the core `spacecan` library, allowing `spacecan-firmware` to compile natively with `#![no_std]` and zero dynamic memory footprint.

### The Feature Isolation Mistake
During the refactoring, `cargo clippy` and `cargo test` began colliding violently. The virtual nodes (`spacecan-virtual`) required the `std` feature (for `tokio`, `std::sync::Mutex`, and network sockets), while the firmware required strict `#![no_std]`. 

By attempting to run `cargo clippy --workspace`, the features "leaked" across the workspace dependencies, breaking both the firmware (due to `std` injection) and the host tests. Furthermore, when I utilized `cargo clippy --fix`, it incorrectly pruned required host-imports because it evaluated the code under `#![no_std]` context where the code was gated behind `#[cfg(feature = "std")]`.

**The Solution:**
I strictly isolated the compilation pipelines. I configured `.github/workflows/ci.yml` to utilize split jobs, explicitly targeting the packages and passing the appropriate `thumbv7em-none-eabihf` target to the firmware job. I also manually restored the pruned imports and wrapped them in `#[cfg(feature = "std")]` to guarantee Clippy analyzed the code accurately depending on the feature flag resolution.

### Testing without `std`
I initially struggled to test the fragmentation logic because standard tests often rely on `std`. I combated this by keeping the `tests` module entirely internal to the `spacecan` crate but running it via host execution (`cargo test -p spacecan`). This allowed the `heapless` logic to be strictly verified for correct payload assembly and off-by-one errors using standard `#[test]` macros without requiring a complex QEMU hardware-in-the-loop setup.
