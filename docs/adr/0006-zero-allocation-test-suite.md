# ADR 0006: Zero-Allocation and Testing

## Status
Accepted

## Context
The `spacecan` library initially utilized the `extern crate alloc` paradigm, leveraging `alloc::vec::Vec` and `BTreeMap` for storing incoming packet fragments and managing service routing. 
During the integration phase with the STM32F4 firmware target, this dynamic allocation approach caused significant compilation failures and architectural flaws. Attempting to solve this with a static, pre-allocated linked-list allocator introduced unacceptable runtime non-determinism and heap-fragmentation risks, which violate strict aerospace and MISRA-C software constraints.

## Decision
1. We mandate the complete removal of the `alloc` crate from the workspace.
2. We adopt the `heapless` crate to define strict compile-time memory boundaries.
3. `PacketData` is constrained to `heapless::Vec<u8, 1024>`.
4. `FrameData` is constrained to `heapless::Vec<u8, 8>`.
5. The test suite will be integrated locally within the `spacecan` crate but executed using standard host compilation targets to verify the mathematical and logical assertions of the zero-allocation assembler without requiring hardware-in-the-loop environments.

## Consequences
**Positive:**
- Absolute deterministic memory usage. The firmware will never suffer from an out-of-memory panic during runtime.
- The firmware binary size is heavily optimized by stripping allocator overhead.
- True `#![no_std]` compliance is guaranteed across all embedded targets.

**Negative:**
- Memory footprints for structs are permanently fixed at compile time. A `SpaceCANPacket` allocates a flat 1024 bytes in memory (or on the stack) even if the payload is only 3 bytes, slightly increasing static RAM usage.
