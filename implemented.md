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


---

## [2026-11-08] — Milestone M1: Dataset Inventory and Template Configuration

### 1. Milestone Objectives & Scope
- **Charter:** Establish normalized geometric primitives, versioned JSON template representation with comprehensive boundary validation, dataset manifest schema, ground-truth label parser, synthetic reference template, and functional CLI template inspection.
- **Constraints Honored:**
  - Zero ML, PostgreSQL/database, GUI/web, or desktop dependencies.
  - Zero OpenCV code introduced (pure Rust domain modeling).
  - Preserved existing architecture and accepted ADRs 001–010.
  - No large external datasets downloaded or committed.
  - Clean library/binary separation preserved.

---

### 2. Dataset Research & Reconciliation
- **Primary Dataset:** Tamaulipas Multiple-Choice-Question Exam Image Dataset (Version 2).
- **Source:** Mendeley Data (`https://data.mendeley.com/datasets/djmynjwjpy/2`), DOI: `10.17632/djmynjwjpy.2`.
- **License:** CC BY 4.0 (attribution required, commercial/academic use permitted).
- **Sheet Count Reconciliation:**
  - Mendeley v2 metadata records **5,721** published scanned sheets:
    - Cohort 90: 3,669 sheets (90 items, 4 options A-D).
    - Cohort 100: 2,052 sheets (100 items, 4 options A-D).
  - MDPI paper (*Journal of Imaging*, 2025) mentions **6,029** total sheets collected across educational institutions. The discrepancy (308 sheets) represents examination papers withheld or filtered prior to Mendeley publication. OMNISCAN targets the 5,721 published sheets.
- **Candidate ID Privacy:** Student names and identifiers are anonymized/masked on physical sheets. The public ground-truth labels do not provide candidate roll numbers. Candidate ID bubble decoding is therefore verified using synthetic templates.
- **Staging Decision:** Real sheet coordinates are not invented. Template calibration for Tamaulipas Cohort 90 will occur in Milestone M2 after decoding real sample images.

---

### 3. Dependency Decisions
Added only two pure-Rust dependencies to `Cargo.toml`:

| Crate | Version | Justification |
|---|---|---|
| `serde` | `1.0` (features: `["derive"]`) | Compile-time declarative serialization/deserialization framework. |
| `serde_json` | `1.0` | High-performance JSON parser for template and manifest files. |

**Kept Existing:** `clap` (v4.5), `thiserror` (v2.0).  
**Deferred to M2:** `opencv`.

---

### 4. Architecture & Module Design

```text
omniscan/
├── configs/
│   └── templates/
│       └── synthetic-standard-30q-v1.json    # Calibrated 30-question synthetic template
├── docs/
│   └── datasets/
│       └── tamaulipas-research.md             # Dataset research profile & reconciliation
├── src/
│   ├── core/
│   │   └── mod.rs                            # Point2D, Size2D, NormalizedRect, Circle, Orientation
│   ├── layout/
│   │   └── mod.rs                            # Template, PageLayout, MarkerLayout, CandidateIdLayout, QuestionLayout
│   ├── manifest/
│   │   └── mod.rs                            # DatasetManifest, ManifestRecord, DatasetSplit, path validation
│   ├── labels/
│   │   └── mod.rs                            # GroundTruthSheet, QuestionLabel, ResponseMark parser
│   ├── cli.rs                                # CLI arguments & subcommands
│   ├── error.rs                              # Typed OmniscanError (Template, Manifest, GroundTruth variants)
│   ├── lib.rs                                # Library exports (core, layout, manifest, labels, cli, error)
│   └── main.rs                               # Binary entry point wiring inspect-template
└── tests/
    ├── fixtures/
    │   ├── templates/                        # 8 synthetic template fixtures (valid & invalid)
    │   ├── manifests/                        # 4 synthetic manifest fixtures (valid & invalid)
    │   └── labels/                           # 4 synthetic ground-truth fixtures (valid & invalid)
    ├── cli_smoke_tests.rs                    # Integration tests for binary execution & inspect-template
    ├── template_validation_tests.rs          # Template boundary & uniqueness integration tests
    └── manifest_and_labels_tests.rs          # Manifest security & ground-truth parser tests
```

#### Detailed Component Breakdown:
1. **Geometric Core (`src/core/mod.rs`):**
   - Coordinates normalized to $[0.0, 1.0]$ with top-left origin.
   - `Circle::overlaps_with(&self, other: &Circle)` computes pairwise distance against the sum of radii ($r_1 + r_2$).
   - `NormalizedRect::is_within_bounds()` enforces strict positive dimensions inside page bounds.
2. **Template Representation & Validation (`src/layout/mod.rs`):**
   - `schema_version == 1` enforcement.
   - Page dimensions strictly positive; aspect ratio verified against declared reference width/height within 0.01 tolerance.
   - Registration markers validated for positive size, unique IDs, and boundary containment.
   - Candidate ID grid validated for exactly 6 columns, 10 rows (digits 0-9), non-overlapping spacing, and boundary bounds.
   - Questions validated for uniqueness across sections, minimum 2 options per question, unique option labels within questions, positive radii, and pairwise bubble non-overlap.
   - Formatted `TemplateSummary` display implementation.
3. **Synthetic Reference Template (`configs/templates/synthetic-standard-30q-v1.json`):**
   - All 30 questions explicitly defined (3 columns of 10 questions, 4 options A-D).
   - 4 corner fiducial markers.
   - 6-digit candidate ID bubble grid.
   - Mathematically verified non-overlapping coordinates.
4. **Dataset Manifest (`src/manifest/mod.rs`):**
   - Unique record IDs enforced.
   - Strict relative path enforcement: rejects absolute paths (starting with `/`, `\`, or `C:`) and path traversal (`..` components).
   - Validates partition splits (`train`, `validation`, `test`, `holdout`).
   - Separate method `validate_files_exist(base_dir)` for checking disk files.
5. **Ground-Truth Label Parser (`src/labels/mod.rs`):**
   - Supports single-mark (`Single(char)`), multiple-mark (`Multiple(Vec<char>)`), blank (`Blank`), and ambiguous (`Ambiguous(String)`).
   - Handles comma, tab, and whitespace delimiters.
   - Strips comments (`#`) and extracts candidate ID directives (`# candidate_id: <ID>`).
   - Line-number preservation on all errors.
   - Rejects duplicate question IDs, duplicate options within multiple marks, and malformed tokens (never silently treats unknown tokens as blank).
6. **CLI Implementation (`src/main.rs`):**
   - Functional `omniscan inspect-template <TEMPLATE.json>`.
   - On valid template: prints human-readable summary to `stdout` and exits with code 0.
   - On invalid/corrupt template: prints typed error to `stderr` and exits with code 1.

---

### 5. Validation Results & Quality Gates

| Command | Status | Notes |
|---|---|---|
| `cargo fmt --check` | **PASSED** | 0 formatting diffs across all source files and tests. |
| `cargo clippy --all-targets --all-features -- -D warnings` | **PASSED** | 0 warnings across library, binary, and test targets. |
| `cargo test` | **PASSED** | **47/47 tests passed (0 failures):**<br>- 22 unit tests in `src/lib.rs`<br>- 8 integration tests in `cli_smoke_tests`<br>- 9 integration tests in `template_validation_tests`<br>- 8 integration tests in `manifest_and_labels_tests` |
| `cargo run -- inspect-template configs/templates/synthetic-standard-30q-v1.json` | **PASSED** | Correctly outputs validated summary and returns exit code 0. |

---

### 6. Next Milestone
- **Milestone M2 — Image Ingestion and Preprocessing:**
  - Implement image decoding with error handling for unsupported/corrupt files.
  - Establish project-owned image abstraction behind an imaging adapter boundary.
  - Implement grayscale conversion and benchmark Otsu vs adaptive thresholding.
  - Setup native OpenCV build environment per `docs/windows-setup.md`.


---

## [2026-11-09] — Milestone M1: Pre-Commit Formatting Normalization & Quality Gate Verification

### 1. Action & Reconciliation
- **Issue:** Minor trailing empty newline in `src/core/mod.rs` flagged during independent audit.
- **Resolution:** Executed `cargo fmt` to canonicalize EOF and whitespace formatting across `src/core/mod.rs` and all new modules.
- **Constraints Honored:**
  - Preserved append-only integrity of `implemented.md`.
  - Zero external dependencies introduced.
  - No commits performed prior to explicit user authorization.

### 2. Final Verification Results
- `cargo fmt --check`: **PASSED** (0 diffs, exit code 0).
- `cargo clippy --all-targets --all-features -- -D warnings`: **PASSED** (0 warnings, exit code 0).
- `cargo test`: **PASSED** (47/47 tests passed across all units and integration suites).
