# Contributing to SpaceCAN

Thank you for your interest in contributing to SpaceCAN. Your contributions help improve spacecraft communication and make the project more robust. Follow this guide to ensure a smooth collaboration.

---

## How to Contribute

### 1. Fork the Repository
Click the Fork button at the top of the repo page to create your copy.

### 2. Clone Your Fork
```sh
git clone https://github.com/your-username/SpaceCAN.git
cd SpaceCAN
```

### 3. Create a New Branch
```sh
git checkout -b feature-name
```
Use a descriptive branch name, like `feature-improve-logging`.

### 4. Make Your Changes
- Follow the existing code structure.
- Write clear, maintainable, and well-documented code.
- Add unit tests in the `/tests` directory when modifying functionality.

### 5. Run Tests
```sh
cargo test
```

### 6. Commit Your Changes
```sh
git add .
git commit -m "Added feature: Improved logging system"
```

### 7. Push to Your Fork
```sh
git push origin feature-name
```

### 8. Submit a Pull Request (PR)
1. Go to the original repo: SpaceCAN
2. Click New Pull Request
3. Select your branch & describe your changes
4. Click Create Pull Request

---

## Code Guidelines

- Follow Rust best practices.
- Use meaningful variable and function names.
- Format code using `cargo fmt`.
- Use `cargo clippy` to catch warnings.
- Write inline documentation (`///`) for functions.
- Handle errors properly using `Result<T, E>` instead of panics.

---

## Issue Reporting

Found a bug or want to request a feature? Open an issue!
1. Go to the Issues tab.
2. Click New Issue.
3. Use a clear title and description.
4. Attach logs/screenshots if needed.

---

## Contribution Roadmap
### Good First Issues:
- Improve documentation
- Add new unit tests
- Optimize existing functions
- Refactor code for better readability

### Advanced Contributions:
- Implement async Rust (`tokio`) for efficiency
- Add multi-node CAN communication
- Improve error handling & recovery

---

## License
By contributing, you agree that your contributions will be licensed under the MIT License.
