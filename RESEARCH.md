# Research Foundation: rust-spacecan

## 1. Domain: Embedded Systems (Cortex-M4 / STM32F4)

### A. Required Knowledge
- **Cortex-M4 Architecture**: NVIC (Nested Vectored Interrupt Controller), SysTick timer, float-abi (hard-float execution), and memory-mapped IO.
- **bxCAN Peripheral**: STM32F4's Controller Area Network hardware module, including mailbox systems, FIFO queues, filter banks, and error management registers.
- **Static Resource Allocation**: Eliminating global allocators (`no_std` without `alloc`) to achieve deterministic memory footprint, utilizing stack-based structures or static pools (`heapless` crate).

### B. Research Questions
1. How do we initialize and service the STM32F4 bxCAN peripheral without blocking core CPU operations?
2. How do we configure bxCAN filter registers to perform hardware-level filtering of CAN IDs based on ECSS standard masks (Node ID, Function ID) to optimize interrupt load?
3. What is the most efficient pattern to share the CAN driver state between interrupt service routines (ISRs) and execution tasks without causing undefined behavior?

### C. Standards & References
- **STMicroelectronics RM0090 Reference Manual**: Detailed bxCAN hardware registers, mailboxes, and filter configurations.
- **ARMv7-M Architecture Reference Manual**: Instruction set, CPU states, and interrupt nesting behavior.

## 2. Domain: Space Communication Standards

### A. ECSS-E-ST-50-15C (Space CAN Bus Protocol)
#### Required Knowledge
- **CAN ID Structuring**: 11-bit CAN identifiers split into:
  - Function ID (bits 7-10)
  - Node ID (bits 0-6)
- **Time Distribution**: Spacecraft Elapsed Time (SCET) and UTC broadcast formats over CAN.
- **Bus Redundancy**: Dual-bus topology (CAN Bus A and CAN Bus B) with automatic physical link switching upon heartbeat timeout.
- **System Synchronization**: Periodic SYNC frame (0x080) for aligning clock offsets across nodes.

#### Research Questions
1. What is the precise timing constraint for redundant bus switching when a node's heartbeat is missed?
2. How can clock drift be mathematically corrected on the responder nodes using the periodic SCET and UTC frames?

### B. ECSS-E-ST-70-41C (Packet Utilization Standard - PUS)
#### Required Knowledge
- **PUS Packet Layout**: Standardized telecommand (TC) and telemetry (TM) packet structures containing APID, packet sequence control, service type, subservice type, and source data.
- **Services to Implement**:
  - `ST01`: Request Verification (verify acceptance, start, progress, and completion of telecommands).
  - `ST03`: Housekeeping (structure periodic parameter acquisition).
  - `ST08`: Function Management (on-board function triggers).
  - `ST17`: Connection Test (ping-pong verification).
  - `ST20`: Parameter Management (reading/writing system registers).

#### Research Questions
1. What is the optimal design for a zero-allocation PUS packet router that supports dynamic registration of service handlers?
2. How do we prevent buffer overflows when serializing arbitrary parameter telemetry (Service 3) into tiny 8-byte CAN payloads?

## 3. Domain: Network Emulation & Portability

### A. Required Knowledge
- **Cross-Platform CAN Simulation**: Emulating standard CAN bus layouts using TCP/UDP sockets.
- **Frame Serialization**: Wrapping CAN frame metadata (ID, data payload, timestamp) inside standard byte arrays for socket transmission.

### B. Research Questions
1. How do we construct a UDP broadcast/multicast simulation that replicates multi-drop CAN bus characteristics (e.g. all nodes receive broadcast frames)?
2. What are the performance and latency profiles of virtual TCP/UDP framing compared to raw Linux SocketCAN sockets?

### C. Existing Solutions
- **SocketCAN (Linux)**: Standard Linux kernel network interface for CAN drivers.
- **Wireshark CAN dissection**: Analyzing CAN traffic via standard network capture tools.

## 4. Key Papers and Standards Docs

1. **ECSS-E-ST-50-15C**: *"Space engineering - CAN bus extension protocol"*, European Cooperation for Space Standardization.
2. **ECSS-E-ST-70-41C**: *"Space engineering - Telemetry and telecommand packet utilization"*, European Cooperation for Space Standardization.
3. **Candidacy of Rust in Space Software**: Studies by ESA and NASA detailing Rust's viability for safety-critical flight applications, focusing on memory safety and concurrency without GC overhead.
