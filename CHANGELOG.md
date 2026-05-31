# Changelog: rust-spacecan

All notable changes to this project will be documented in this file

## [0.1.1] - 2026-05-31

### Added
- Created host unit testing mock module `defmt_mock.rs` under `spacecan/src/tests/` to support standard `cargo test` builds.
- Added target-specific compile gates `#[cfg(target_os = "linux")]` to `spacecan-virtual/controller.rs` and `responder.rs`.
- Added platform checks to `spacecan-virtual/build.rs` to skip linking `c.lib` on Windows.
- Integrated `test = false` configurations to `spacecan-firmware/Cargo.toml` to disable host unit-testing.

### Changed
- Configured target-specific dependencies for `socketcan` in `spacecan-virtual/Cargo.toml`.
- Gated `defmt` format implementations in `spacecan/src/protocol.rs` to compile only under `#[cfg(all(feature = "defmt", not(test)))]`.
- Enabled the `defmt` feature flag on `spacecan` dependency inside `spacecan-firmware/Cargo.toml`.

### Removed
- Deleted orphaned duplicate files `reciever.rs`, `controller.rs`, and `parser.rs` from `spacecan/src/`.
- Removed redundant `[[bin]]` configuration and `src/main.rs` file from the core `spacecan` crate.
- Removed duplicate `examples/firmware.rs` from `spacecan-firmware`.
- Removed unconditional `#[global_allocator]` and `init_allocator` from the core library `lib.rs` (decoupling allocators from library logic).
- Cleaned up duplicate key `strip = "debuginfo"` and invalid keys `build-std`/`build-std-features` from `spacecan-firmware/Cargo.toml`.
