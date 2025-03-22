# 🚀 SpaceCAN Examples

This directory contains **example scripts** demonstrating how to use the SpaceCAN protocol in Rust.

---

## 📌 **Structure**
```
examples/
│-- README.md
│-- basic/               # Simple SpaceCAN usage examples
│   ├── send_can.rs      # Example: Sending a CAN frame
│   ├── receive_can.rs   # Example: Receiving a CAN frame
│   ├── sync_example.rs  # Example: Synchronization message handling
│   ├── heartbeat.rs     # Example: Heartbeat monitoring
│-- packet/              # Packet fragmentation & reassembly examples
│   ├── split_packet.rs   # Example: Splitting large packets into CAN frames
│   ├── reassemble_packet.rs # Example: Reassembling packets from received frames
│   ├── full_packet_demo.rs  # Example: End-to-end packet transmission and reception
│-- services/
│   ├── packet_service.rs   # Example: A Service for Handling Packets
│   ├── service_full_demo.rs # Example: Full Service Demo for Packet Transmission & Reception
│   ├── service_splitter.rs  # Example: Service for Splitting Large Packets

```

---

## 🚀 **How to Run the Examples**

### **🔹 Prerequisites**
Ensure you have:
- Rust installed (`rustc --version` to check)
- A CAN interface (or a software CAN simulator)
- Built the SpaceCAN library:
  ```sh
  cargo build --release
  ```

### **🔹 Running an Example**
To run an example, use:
```sh
cargo run --example split_packet
```
Replace `split_packet` with the desired example.