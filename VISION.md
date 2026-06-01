# Project Vision: rust-spacecan

`rust-spacecan` is an open-source Rust implementation of the ECSS CAN bus application layer and Packet Utilization Standard, designed for cubesat and small satellite flight software.

## 1. Problem Statement

Developing flight software for cubesats and small satellites is notoriously complex, expensive, and error-prone:
- **Proprietary Bottlenecks**: Satellite communication and command protocols historically rely on closed-source, vendor-locked software stacks.
- **Hardware Dependency**: Software validation is typically tied to physical hardware-in-the-loop (HIL) platforms, slowing down continuous integration (CI) and rapid testing.
- **Memory Safety Risks**: High-reliability protocols are traditionally written in C/C++, exposing space missions to fatal runtime failures (e.g., buffer overflows, memory leaks, and null pointer dereferences) in harsh, remote environments.

`rust-spacecan` addresses these challenges by providing an open-source, memory-safe, hardware-decoupled Rust implementation of the European Cooperation for Space Standardization (ECSS) CAN bus application layer (`ECSS-E-ST-50-15C`) and Packet Utilization Standard (`ECSS-E-ST-70-41C`).

## 2. Why it Matters

In aerospace engineering, software failures are mission-ending. Space agencies and commercial operators are adopting Rust to guarantee:
1. **Memory Safety**: Compile-time guarantees against data races, memory leaks, and buffer overflows without the overhead of a garbage collector.
2. **Deterministic Execution**: Hard-bounded resource allocations (`no_std`) appropriate for real-time aerospace firmware.
3. **Decoupled Architecture**: Software that can be compiled and verified natively on a developer's desktop (using mock transports and virtual topologies) and flashed unchanged to target microcontrollers (such as the STM32F4).

## 3. Target Audience

- **SmallSat/CubeSat Missions**: Academic groups and commercial startups looking for a standard-compliant, memory-safe flight software stack.
- **Embedded Rust Practitioners**: Systems developers seeking design patterns for decoupling hardware peripherals from business logic.

## 4. Engineering Concepts Demonstrated

This project covers the following systems architecture areas:
- **Aerospace Standards Compliance**: Conformance to `ECSS-E-ST-50-15C` (CAN framing, heartbeat, sync, SCET time synchronization) and `ECSS-E-ST-70-41C` (Services 1, 3, 8, 17, and 20).
- **Hardware-Software Decoupling**: Separation of the physical/virtual CAN peripheral through clean Rust traits.
- **Zero-Allocation Protocol Stack**: Packet fragmentation/reassembly without relying on a global heap, preventing runtime out-of-memory crashes.
- **Cross-Platform Verification**: A unified codebase that runs in virtual multi-node simulations (via TCP/UDP sockets on Windows/Linux) and on bare-metal hardware (`stm32f4xx-hal` target).
- **Observability**: Structured diagnostics (`defmt` for embedded targets, `tracing` for virtual nodes).

## 5. Design Differentiators

Unlike existing projects that are either purely theoretical or locked to specific hardware:
- **Dual-Execution Model**: Transitions between virtual networks (runs on Windows/Linux out of the box) and bare-metal targets (STM32F4).
- **Developer Experience**: Modern tooling integration (Renode emulation, local network simulation, and GitHub Actions CI verification).
- **Standardized Foundation**: One of the few open-source implementations of ESA standards written in idiomatic Rust.
