# Research

This document outlines the technical research governing the design of the `spacecan` protocol stack.

## 1. Controller Area Network (CAN)
CAN 2.0B was selected as the underlying transport layer due to its prevalence in both automotive and modern aerospace systems. It offers deterministic bus arbitration, differential signaling for noise immunity, and CRC error checking.
- **Constraints**: CAN payload data is strictly limited to 8 bytes per frame.
- **Solution**: To transmit telemetry that exceeds 8 bytes, a fragmentation and assembly protocol must sit on top of the CAN frames.

## 2. ECSS PUS Standard (ECSS-E-ST-70-41C)
The Packet Utilization Standard (PUS) is a European Cooperation for Space Standardization protocol. It defines standard application-level services for telecommanding and telemetry.
- **ST01 (Request Verification)**: Confirms receipt and execution status of telecommands.
- **ST03 (Housekeeping)**: Standardized reporting of thermal, electrical, and operational health.
- **ST08 (Function Management)**: High-level system triggers.
- **ST09 (Time Management)**: Used for network time synchronization and heartbeat mechanisms.
- **ST17 (Test)**: A ping-pong mechanism to verify connection integrity.
- **ST20 (Parameter Management)**: On-orbit adjustment of system parameters.

## 3. Real-Time Embedded Rust constraints
When targeting Cortex-M (specifically the STM32F4 Series), the software must be deterministic.
- **`no_std` Environment**: The Rust Standard Library cannot be used. We must rely on `core` and hardware-specific crates.
- **Zero Allocation**: Dynamic memory (e.g., `alloc::vec::Vec`) inherently suffers from heap fragmentation and non-deterministic execution times. We utilized the `heapless` crate to define strict, stack-allocated capacities for all data structures (e.g., 1024 bytes max for a `PacketData` struct).

## 4. Hardware and Concurrency
For the STM32F4 `spacecan-firmware`, standard threads are unavailable. 
- Research into embedded concurrency led to **RTIC (Real-Time Interrupt-driven Concurrency)**. RTIC allows us to bind specific tasks to hardware interrupts and schedule software tasks deterministically without a heavy RTOS.
