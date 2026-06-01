# Metrics: rust-spacecan

## 1. Codebase Size Metrics (Phase 1)

| Crate / Directory | Files Count | Lines of Code (LOC) | Notes |
| :--- | :--- | :--- | :--- |
| `spacecan` (Library) | 16 | ~1,600 | Pure `no_std` core protocol and PUS services. |
| `spacecan-firmware` | 3 | ~100 | Target-specific firmware bindings. |
| `spacecan-virtual` | 3 | ~450 | CLI simulator environment. |
| **Workspace Total** | **22** | **~2,150** | Optimized footprint after cleanup. |

### Code Sanitization Footprint
- **LOC Deleted**: ~950 lines of duplicate, unused source files (`reciever.rs`, `controller.rs`, `parser.rs`, and duplicate example/runner files).
- **Redundancy Reduction**: ~30% reduction in source file clutter.

---

## 2. Test & Build Metrics

| Metric | Target | Status | Notes |
| :--- | :--- | :--- | :--- |
| **Cargo Check** | Workspace | **PASS** | Compilation succeeds on Windows & Linux hosts. |
| **Host Unit Tests** | `spacecan` library | **6 / 6 PASS** | All core protocol tests run on host. |
| **Linker Errors** | Workspace | **0** | Linker issues (`c.lib`, `defmt` stubs) resolved. |
| **Warnings count** | Workspace | **35** | Mostly dead code in service stubs (to be fixed). |
