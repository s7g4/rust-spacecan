# 🚀 SpaceCAN - Rust Implementation for LibreCube

## **🔹 About SpaceCAN**
SpaceCAN is a communication protocol designed for **small spacecraft systems**. It provides a **lightweight, reliable, and efficient** way to exchange commands and telemetry data between subsystems in space applications.

This repository contains a **Rust-based implementation** of the SpaceCAN protocol for **LibreCube**, aiming to support embedded systems and spacecraft communications.

---

## **✨ Features**
✅ Implements SpaceCAN protocol in Rust  
✅ Supports CAN communication for spacecraft systems  
✅ Efficient message handling with low-latency  
✅ Concurrency support using `std::sync::Mutex`  
✅ Error handling using `thiserror` for robustness  
✅ Embedded-friendly design (no_std compatible)  
✅ Unit-tested for reliability  
✅ Benchmarked for performance  

---

## **📌 Project Structure**
```
SpaceCAN/
│-- src/
│   ├── primitives/       # Core communication components
│   │   ├── can_frame.rs        # CAN Frame struct & serialization
│   │   ├── heartbeat.rs        # Heartbeat signal processing
│   │   ├── network.rs          # CAN network handling
│   │   ├── packet.rs           # Packet fragmentation & reassembly
│   │   ├── sync.rs             # Sync message handling
│   │   ├── timer.rs            # Periodic task scheduling
│   ├── services/        # Service layer components
│   │   ├── core.rs              # Packet routing & processing
│   │   ├── ST01_request_verification.rs  # Request verification
│   │   ├── ST03_housekeeping.rs          # Housekeeping service
│   │   ├── ST08_function_management.rs   # Function management
│   │   ├── ST17_test.rs                  # System test service
│   │   ├── ST20_parameter_management.rs   # Parameter management
│   ├── transport/       # Low-level transport layer
│   │   ├── base.rs              # Bus implementation
│   │   ├── buffer.rs            # Frame buffering
│-- tests/           # Unit test suite
│   ├── test_base.rs         # Unit tests for Bus implementation
│   ├── test_can_frame.rs    # Unit tests for CAN frames
│   ├── test_core.rs         # Unit tests for packet processing
│   ├── test_heartbeat.rs    # Unit tests for heartbeat service
│   ├── test_packet.rs       # Unit tests for packet fragmentation
│   ├── test_sync.rs         # Unit tests for sync processing
│   ├── test_timer.rs        # Unit tests for timer module
│-- README.md               # Project documentation
│-- CONTRIBUTING.md         # Contribution guidelines
│-- LICENSE.md              # Project license
│-- Cargo.toml              # Rust package manager file
```

---

## **🚀 Getting Started**

### **🔹 Prerequisites**
Ensure you have the following installed:
- Rust & Cargo (`rustc --version` to check)
- CAN Interface (or a software CAN simulator)
- `cargo` package manager

### **🔹 Installation**
Clone the repository and build the project:
```sh
git clone https://github.com/N7GG4/SpaceCAN.git
cd SpaceCAN
cargo build --release
```

### **🔹 Running the Example**
```sh
cargo run --example demo
```

---

## **📡 Usage**
```rust
use spacecan::controller::SpaceCANController;

fn main() {
    let mut controller = SpaceCANController::new();
    let data = [0x12, 0x34, 0x56, 0x78];
    controller.transmit(data).unwrap();
}
```

---

## **🛠 Roadmap & Future Improvements**
📌 Implement advanced error recovery  
📌 Support multi-node CAN communication  
📌 Improve efficiency with buffer optimizations  
📌 Integrate with LibreCube’s test framework  
📌 Migrate thread-based tasks to async Rust (`tokio`)  

---

## **📜 Contributing**
We welcome contributions! To contribute:
1. Fork the repository
2. Create a new branch (`git checkout -b feature-xyz`)
3. Commit your changes (`git commit -m "Added new feature"`)
4. Push to your branch (`git push origin feature-xyz`)
5. Open a Pull Request 🚀

See [CONTRIBUTING.txt](CONTRIBUTING.txt) for detailed guidelines.

---

## **📄 License**
This project is licensed under the **MIT License**. See [LICENSE.txt](LICENSE.txt) for details.

---

## **📢 Contact & Community**
For questions and discussions, join the **LibreCube Community**:
- **Website:** [LibreCube Official Site](https://librecube.gitlab.io/)
- **GSoC Proposal:** [Your GSoC Proposal Link]
- **Email:** [Your Email Here]

🚀 **Let's build the future of space communication together!**
