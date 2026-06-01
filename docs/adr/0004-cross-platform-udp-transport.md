# ADR 0004: Cross-Platform UDP Multicast Transport

## Status
Accepted

## Context
The virtual simulation nodes (`spacecan-virtual/controller.rs` and `responder.rs`) originally relied on Linux's `socketcan` interface (`vcan0`). This created a hard OS constraint, forcing developers on Windows and macOS to rely on Linux VMs or WSL to compile and run the simulation stack. As a modern aerospace framework, the simulation environment must be platform-agnostic to support diverse engineering teams.

## Decision
We will replace `socketcan` with a standard **UDP Multicast** network layer (`224.0.0.123:5000`) for all virtual node communication. We will utilize the `socket2` crate to enable `SO_REUSEADDR` and `SO_REUSEPORT`, allowing multiple virtual nodes to bind to the same UDP port on a single machine.

## Consequences
- **Positive**: The `spacecan-virtual` stack compiles and runs natively on Windows, macOS, and Linux without any kernel-level dependencies.
- **Positive**: Lays the foundation for distributed simulation, where virtual nodes can run on entirely separate physical machines on the same local network.
- **Negative**: Adds a tiny layer of IP overhead compared to raw SocketCAN frames, though entirely negligible for software-in-the-loop (SIL) testing.
