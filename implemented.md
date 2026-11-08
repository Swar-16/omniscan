# OMNISCAN — Implementation & Operational Ledger

This file serves as the persistent, chronological record of implementation decisions, architectural patterns, environment setups, code additions, and improvisations in OMNISCAN.

> **CRITICAL RULE FOR ALL AGENTS:**  
> **NEVER erase earlier lines or sections of this file.** Always append new milestone logs, decisions, or improvisations to the bottom of this document. Every future agent working on this repository must read this file to maintain context, design continuity, and architectural integrity.

---

## [2026-11-07] — Milestone M0: Rust Project and Environment Foundation

### 1. Milestone Objectives & Scope
- **Charter:** Establish the foundational Rust CLI repository, environment tooling, CI pipeline, top-level typed error handling, and discoverable CLI interface for OMNISCAN.
- **Constraints Honored:**
  - No OMR algorithms implemented.
  - No datasets downloaded or committed.
  - No database (PostgreSQL/SQLite) introduced (CLI remains local and file-based).
  - No Machine Learning dependencies.
  - No GUI, desktop, or web framework dependencies.
  - Preserved accepted Architecture Decision Records (ADRs 001–010).
  - Zero unrelated broad refactors.

---

### 2. Host Environment Reconnaissance & Setup
- **Host OS:** Windows 11 / Windows 10 (x86_64-pc-windows-msvc).
- **Compilers & Pre-existing Tools Identified:**
  - MSVC C++ Build Tools: Visual Studio Community 2026 (`v145` / `14.51.36231`) with `cl.exe` and `link.exe`.
  - LLVM / Clang: Clang 23.1.3 installed at `C:\Program Files\LLVM\bin\clang.exe` with `libclang.dll` present (critical for `clang-sys` / `bindgen` used by native OpenCV Rust bindings).
  - CMake: CMake 3.31.4 installed at `C:\Users\SWARNAVA\AppData\Roaming\Python\Python312\Scripts\cmake.exe`.
  - Git: Git 2.47.0 (working tree clean on branch `chore/m0-rust-cli-foundation`).
- **Rust Toolchain Installation:**
  - Rust was initially absent from the environment.
  - Installed official stable Rust toolchain via `rustup-init.exe`:
    - `rustc 1.99.1 (58cf8f9c3 2026-10-13)`
    - `cargo 1.99.1 (5f94df478 2026-08-27)`
    - `rustfmt 1.10.0-stable (58cf8f9c35 2026-10-13)`
    - `clippy 0.1.99 (58cf8f9c35 2026-10-13)`
  - Configured user `PATH` to include `%USERPROFILE%\.cargo\bin`.
  - Pinned toolchain in repository via `rust-toolchain.toml` with components `rustfmt` and `clippy`.

---

### 3. Native OpenCV Analysis & Strategic Decisions
- **Discovery Status:**
  - Python's `cv2` (v5.0.0) exists in user site-packages, but only contains pre-compiled Python `.pyd` and private DLLs. It does **not** provide C++ development headers (`opencv2/core.hpp`), CMake package files (`OpenCVConfig.cmake`), or static import libraries (`opencv_world.lib`).
  - No native OpenCV SDK was configured in system environment variables (`OPENCV_DIR` and `LIBCLANG_PATH` were unset).
- **Architectural Decision on OpenCV Dependency for M0:**
  - *Problem:* Declaring `opencv` as an unconditional dependency in `Cargo.toml` would break clean checkouts and GitHub Actions CI (`windows-latest`), because the `opencv` build script attempts to compile bindings against native C++ headers at build time.
  - *Solution:* In M0, keep `Cargo.toml` minimal with pure Rust dependencies (`clap` and `thiserror`). Document native OpenCV prerequisites and Windows environment variable configuration in `docs/windows-setup.md`. Native OpenCV integration is staged for **M2 (Image Ingestion and Preprocessing)** behind an imaging adapter boundary (as mandated by `AGENTS.md` and ADR-002 / ADR-005).

---

### 4. Dependency Decisions
Only justified, strictly necessary dependencies were added to `Cargo.toml`:

| Crate | Version | Role / Justification |
|---|---|---|
| `clap` | `4.5` (features: `["derive"]`) | Declarative, compile-time verified CLI argument parsing, subcommands, and auto-generated `--help` / `--version`. |
| `thiserror` | `2.0` | Ergonomic, zero-cost derive macros for typed domain error enumerations (`OmniscanError`) without panics. |

**Explicitly Deferred Dependencies:**
- `serde` / `serde_json`: Deferred to M1 (when template JSON parsing begins).
- `opencv`: Deferred to M2 (when image ingestion begins).

---

### 5. Codebase Structure & Implemented Files

```text
omniscan/
├── .github/
│   └── workflows/
│       └── ci.yml                 # CI pipeline: fmt, clippy (-D warnings), cargo test
├── docs/
│   └── windows-setup.md           # Guide for MSVC, LLVM Clang, rustup, and native OpenCV SDK
├── src/
│   ├── lib.rs                     # Library crate root: exports cli and error modules
│   ├── main.rs                    # Binary entry point: executes CLI, formats errors, exit codes
│   ├── cli.rs                     # Clap CLI parser defining subcommands matching cli-specification.md
│   └── error.rs                   # Typed OmniscanError enum and Result type alias
├── tests/
│   └── cli_smoke_tests.rs         # Integration tests verifying CLI help, version, subcommands, errors
├── Cargo.toml                     # Minimal Rust project manifest
├── Cargo.lock                     # Locked dependency graph (23 crates)
├── implemented.md                 # Agent persistent ledger (THIS FILE)
├── README.md                      # Updated with actual build/verification commands & status
└── rust-toolchain.toml            # Pinned stable MSVC toolchain specification
```

#### Detailed Breakdown of Implementations:
1. **`src/error.rs`:**
   - Defines `OmniscanError` enum with typed variants:
     - `Io(#[from] std::io::Error)`
     - `Config(String)`
     - `ImageProcessing(String)`
     - `Template(String)`
     - `Alignment(String)`
     - `Ambiguity(String)`
     - `Scoring(String)`
     - `Cli(String)`
   - Defines `pub type Result<T> = std::result::Result<T, OmniscanError>;`.
   - Includes unit tests for error formatting and `io::Error` conversion.
2. **`src/cli.rs`:**
   - Defines `Cli` struct with derive parser and subcommands reflecting `docs/cli-specification.md`:
     - `validate-image <IMAGE>`
     - `inspect-template <TEMPLATE>`
     - `process <IMAGE> --template <TEMPLATE> [--answer-key <KEY>] [--output <DIR>]`
     - `batch <INPUT_DIR> --template <TEMPLATE> [--answer-key <KEY>] --output-dir <DIR>`
     - `evaluate <RESPONSES> --answer-key <KEY>`
     - `benchmark <MANIFEST> --template <TEMPLATE>`
   - Includes unit tests verifying argument parsing for each subcommand.
3. **`src/main.rs`:**
   - Implements `fn run(cli: Cli) -> Result<()>` returning typed errors.
   - Routes commands to designated handler stubs.
   - Formats progress/diagnostics to `eprintln!` (stderr) and results/stubs to `println!` (stdout), preserving separation of machine-readable output.
   - Maps errors to `ExitCode::FAILURE` and successes to `ExitCode::SUCCESS`.
4. **`tests/cli_smoke_tests.rs`:**
   - Executes the compiled `omniscan` binary using `env!("CARGO_BIN_EXE_omniscan")` (standard Cargo zero-dependency test runner).
   - Verifies `--help`, `-h`, `--version`, subcommand `--help` for `validate-image` and `process`, and non-zero exit on unknown options.

---

### 6. Validation Commands and Results

| Command | Purpose | Result | Notes |
|---|---|---|---|
| `cargo fmt --check` | Enforce Rust standard formatting | **PASSED** | Exit code 0, 0 diffs |
| `cargo clippy --all-targets --all-features -- -D warnings` | Enforce zero linter warnings | **PASSED** | Exit code 0, 0 warnings |
| `cargo test` | Execute all unit and integration tests | **PASSED** | 11 tests passed (5 unit tests in `src/lib.rs`, 6 integration tests in `tests/cli_smoke_tests.rs`, 0 failures) |
| `cargo run -- --help` | Verify root help display | **PASSED** | Displays application description, usage, subcommands, and flags |
| `cargo run -- validate-image test.png` | Verify command execution stub | **PASSED** | Diagnostic to stderr, status output to stdout |
| `cargo run -- inspect-template t.json` | Verify template inspect stub | **PASSED** | Diagnostic to stderr, status output to stdout |

---

### 7. Known Environment Limitations & Boundaries
1. **OpenCV Native C++ Binding:**
   - Deferred until M2. At M2, the host machine will require the official OpenCV 4.x Windows release extracted to disk, with `OPENCV_DIR` and `PATH` configured as documented in `docs/windows-setup.md`.
2. **Subcommand Stubs:**
   - Subcommands in M0 validate arguments and print clear milestone dispatch stubs. Actual logic (image decode, template calibration, bubble classification, grading) begins in M1–M7.

---

### 8. Recommended Next Milestone
- **Milestone M1 — Dataset Inventory and Template Configuration:**
  - Tasks:
    1. Dataset manifest format and label parser.
    2. Document dataset license and access details.
    3. Define normalized template geometry JSON schema.
    4. Provide initial template configuration for the chosen sheet cohort.
    5. Add synthetic test fixtures for template validation (without downloading large datasets).



---

## [2026-11-08] — Milestone M0: Formatting Normalization (`cargo fmt`)

### 1. Action & Motivation
- **Issue:** `cargo fmt --check` reported trailing whitespace/newline diffs in `src/error.rs`, `src/lib.rs`, and `src/main.rs`.
- **Resolution:** Executed `cargo fmt` to canonicalize file endings and remove trailing empty lines across all Rust source files.

### 2. Validation
- `cargo fmt --check`: **PASSED** (0 diffs, exit code 0).
- `cargo clippy --all-targets --all-features -- -D warnings`: **PASSED** (0 warnings, exit code 0).
- `cargo test`: **PASSED** (11 tests passed: 5 unit tests, 6 integration tests, 0 failures).
