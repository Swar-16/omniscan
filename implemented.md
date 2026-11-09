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


---

## [2026-11-09] — Milestone M2 Phase 2A: OpenCV Dependency Integration & Project-Owned Image Abstractions

### 1. Phase A Standalone Spike & Environment Verification
- **Verified Environment:**
  - Official OpenCV Windows SDK `4.10.0` extracted to `C:\opencv`.
  - Rust crate `opencv = "0.102.0"` with generator `0.108.0` (resolves AST generation panics encountered with OpenCV 4.10 and LLVM 23 on older 0.93 bindings).
  - Native library link target: `opencv_world4100.lib` / `opencv_world4100.dll`.
  - Feature selection: `default-features = false`, `features = ["imgcodecs", "imgproc"]` (avoids unused `dnn`, `gapi`, `videoio`, `highgui`).
- **Spike Execution in `scratch/opencv_spike`:**
  - Verified binding generation, static library linking, PNG decoding, grayscale conversion, Otsu binarization, and runtime dynamic loading under Windows 11.

### 2. Dependency Integration (`Cargo.toml`)
- Added `opencv = { version = "0.102", default-features = false, features = ["imgcodecs", "imgproc"] }` to `omniscan/Cargo.toml`.
- Generated and locked clean dependencies in `omniscan/Cargo.lock`.

### 3. Project-Owned Image Abstractions (`src/imaging/`)
- **`PixelFormat` Enum:**
  - Supported formats: `Gray8` (1 channel), `Rgb8` (3 channels), `Bgr8` (3 channels).
  - Provided `channels(&self) -> usize` helper.
- **`ImageBuffer` Struct:**
  - Fully encapsulated layout with private fields: `width: u32`, `height: u32`, `format: PixelFormat`, `stride: usize`, `data: Vec<u8>`.
  - Hardened constructors with checked arithmetic:
    - Rejects zero dimensions and dimensions exceeding `MAX_IMAGE_DIMENSION = 15_000` pixels.
    - Rejects allocations exceeding `MAX_IMAGE_PIXEL_BYTES = 120 * 1024 * 1024` (120 MB decompressed pixel cap).
    - Enforces row stride invariant (`stride >= width * channels`).
    - Enforces exact buffer capacity matching `stride * height`.
  - Safe accessors: `width()`, `height()`, `format()`, `stride()`, `channels()`, `as_slice()`, `into_vec()`, `get_pixel(x, y)`.
  - Zero `unsafe` blocks.
- **`GrayImage` Struct:**
  - Dedicated grayscale wrapper around `ImageBuffer` enforcing `format == PixelFormat::Gray8`.
- **`BinaryImage` Struct:**
  - Dedicated binarized wrapper around `ImageBuffer`.
  - Enforces canonical OMNISCAN binarization invariant at construction:
    - Foreground = `0` (dark pencil/pen mark).
    - Background = `255` (unmarked sheet).
    - Any intermediate or unexpected pixel byte is rejected with `ImageError::InvalidBinaryPixel { value, offset }`.
  - Semantic boolean queries: `is_foreground(x, y) -> Option<bool>` and `is_background(x, y) -> Option<bool>`.
- **Zero OpenCV Leakage Boundary:**
  - OpenCV `Mat` and FFI primitives are strictly isolated. No OpenCV types appear in `ImageBuffer`, domain modules, or public APIs.
  - Re-exported cleanly in `src/lib.rs`: `ImageBuffer`, `GrayImage`, `BinaryImage`, `PixelFormat`.

### 4. Typed Error Handling (`src/error.rs`)
- Introduced dedicated `ImageError` enum:
  - `InvalidDimensions { width, height, max }`
  - `DimensionOverflow { width, height, channels }`
  - `AllocationTooLarge { bytes, max }`
  - `BufferLengthMismatch { expected, actual }`
  - `InvalidStride { width, channels, actual_stride, min_stride }`
  - `InvalidBinaryPixel { value, offset }`
- Integrated as `OmniscanError::Image(ImageError)` with `From<ImageError>` implementation.

### 5. CI Automation (`.github/workflows/ci.yml`)
- Configured Windows CI with `actions/cache@v4` caching `C:\opencv`.
- Fallback automated PowerShell download and extraction of official OpenCV 4.10.0 release.
- Added environment setup step setting `OPENCV_INCLUDE_PATHS`, `OPENCV_LINK_PATHS`, `OPENCV_LINK_LIBS`, `LIBCLANG_PATH`, and updating `PATH`.

### 6. Validation Results & Quality Gates
- `cargo fmt --check`: **PASSED** (0 formatting diffs).
- `cargo clippy --offline --all-targets --all-features -- -D warnings`: **PASSED** (0 warnings).
- `cargo test --offline --all-targets`: **PASSED** (62/62 tests passing, 0 failures):
  - 37 unit tests in `src/lib.rs` (including 15 new imaging tests and 2 error tests).
  - 8 integration tests in `cli_smoke_tests`.
  - 8 integration tests in `manifest_and_labels_tests`.
  - 9 integration tests in `template_validation_tests`.

### 7. Milestone M2 Phase Status
- **Current Status:** Phase 2A complete and verified.
- **Next Phase:** Phase 2B (Native OpenCV Image Decoding Adapter & Verification Pipeline).


---

## [2026-11-09] — Milestone M2 Phase 2B: Native OpenCV Image Decoding Adapter & Verification Pipeline

### 1. Adapter Architecture (`src/imaging/opencv_adapter.rs`)
- **Strict Isolation Boundary:**
  - OpenCV `Mat`, FFI primitives, and raw OpenCV errors are encapsulated entirely within `src/imaging/opencv_adapter.rs`.
  - Callers receive project-owned abstractions: [`ImageBuffer`](file:///C:/C/Sem-7/Digital_Image_Processing/omniscan/src/imaging/buffer.rs#L34-L177) and [`GrayImage`](file:///C:/C/Sem-7/Digital_Image_Processing/omniscan/src/imaging/buffer.rs#L182-L260).
  - Concrete public API without speculative traits:
    - `decode_image(path: &Path) -> Result<ImageBuffer, ImageError>`
    - `decode_image_with_order(path: &Path, color_order: ColorOrder) -> Result<ImageBuffer, ImageError>`
    - `decode_gray_image(path: &Path) -> Result<GrayImage, ImageError>`
    - `is_supported_format(path: &Path) -> bool`
    - `ColorOrder` enum (`Rgb`, `Bgr`).

### 2. Supported Formats & Verified Behavior
- **PNG:** 8-bit single-channel grayscale (`CV_8UC1`), 8-bit 3-channel BGR/RGB (`CV_8UC3`), and 8-bit 4-channel BGRA (`CV_8UC4`).
- **JPEG:** 8-bit grayscale (`CV_8UC1`) and 8-bit 3-channel color (`CV_8UC3`).
- **TIFF:** Single-page 8-bit grayscale and 8-bit color. Multi-page TIFFs are detected and rejected.

### 3. Pixel Format Decisions & Channel Ordering
- **Canonical RGB Order:**
  - By default, color images are decoded to standard `PixelFormat::Rgb8`. Pixel copying performs an in-flight byte swap from OpenCV's native BGR layout (`[B, G, R]` -> `[R, G, B]`) without allocating intermediate OpenCV `Mat` structures.
- **Native BGR Order:**
  - When callers explicitly select `ColorOrder::Bgr`, the native layout is preserved as `PixelFormat::Bgr8`.
- **Grayscale Handling:**
  - Grayscale images decode to `PixelFormat::Gray8`.
  - `decode_gray_image` enforces the `Gray8` format; attempting to call it on color images returns a typed `ImageError::ExpectedGrayscale { path, channels }` error (since grayscale conversion algorithms belong to Phase 2C).

### 4. Edge Cases, Invariants & Error Handling
- **Alpha Compositing Against White:**
  - 4-channel BGRA images are normalized by compositing each pixel against a pure white background `(255, 255, 255)` using integer arithmetic:
    `C_out = ((C_src * A + 255 * (255 - A) + 127) / 255) as u8`.
  - Transparency is never silently discarded.
- **16-Bit Depth Rejection:**
  - `mat.depth() != CV_8U` (e.g. `CV_16U = 2`) is rejected with `ImageError::UnsupportedDepth { depth }`.
- **Unsupported Formats:**
  - Extensions outside `.png`, `.jpg`, `.jpeg`, `.tif`, `.tiff` (e.g. `.bmp`, `.gif`, `.webp`, `.pdf`) are rejected with `ImageError::UnsupportedFormat { path, format }`.
- **Multi-Page TIFF Rejection:**
  - TIFF files are decoded using `imreadmulti`. If `pages.len() > 1`, decoding immediately aborts with `ImageError::MultiPageNotSupported { path, pages }`.
- **Missing & Corrupted Files:**
  - Missing paths return `ImageError::FileNotFound { path }`.
  - Corrupted, empty, or unparseable binary streams return `ImageError::DecodeFailed { path, message }`.
- **EXIF Orientation:**
  - OpenCV's `IMREAD_UNCHANGED` preserves raw pixel raster order directly from the storage container and avoids applying automatic EXIF orientation transforms. This prevents lossy or unverified rotation/resampling prior to subsequent sheet alignment and homography stages.

### 5. Memory Safety & Resource Safeguards
- **Pre-Allocation Validation:**
  - Dimensions are verified strictly before destination memory allocation: `0 < width <= MAX_IMAGE_DIMENSION (15,000)` and `0 < height <= MAX_IMAGE_DIMENSION (15,000)`.
  - Checked multiplication validates row stride (`width * channels`) and total pixel capacity (`stride * height`) against integer overflow.
  - Total required allocation is verified to not exceed `MAX_IMAGE_PIXEL_BYTES (120 MiB)` before calling `Vec::with_capacity`.
- **Process Memory Distinction:**
  - The 120 MiB cap bounds the single project-owned `ImageBuffer`. Peak process memory during decoding briefly includes OpenCV's internal decoded buffer in addition to the project-owned copy.

### 6. Validation Results & Quality Gates
- `cargo fmt --check`: **PASSED** (0 formatting diffs).
- `cargo clippy --offline --all-targets -- -D warnings`: **PASSED** (0 warnings across all targets without `--all-features`).
- `cargo test --offline --all-targets`: **PASSED** (**83/83 tests passing**, 0 failed, 0 ignored):
  - **48** unit tests in `src/lib.rs` (37 prior + 11 new adapter unit tests).
  - **10** integration tests in `tests/image_decoding_tests.rs`.
  - **8** integration tests in `tests/cli_smoke_tests.rs`.
  - **8** integration tests in `tests/manifest_and_labels_tests.rs`.
  - **9** integration tests in `tests/template_validation_tests.rs`.
- `cargo tree -e features -p opencv`: **PASSED** (only minimal features `imgcodecs` and `imgproc` enabled).

### 7. Milestone M2 Phase Status
- **Current Status:** Phase 2B complete and verified.
- **Next Phase:** Phase 2C (Grayscale Conversion, Classical Binarization Benchmarks & Preprocessing Pipeline).


---

## [2026-11-09] — Milestone M2 Phase 2C: Grayscale Conversion & Classical Thresholding Engine

### 1. Preprocessing Engine Architecture (`src/imaging/preprocessing.rs`)
- **Encapsulated Preprocessing Boundary:**
  - OpenCV `Mat` allocations and `imgproc` algorithms (`cvt_color`, `threshold`, `adaptive_threshold`) remain strictly internal to `src/imaging/preprocessing.rs`.
  - Inputs and outputs strictly utilize project-owned abstractions: [`ImageBuffer`](file:///C:/C/Sem-7/Digital_Image_Processing/omniscan/src/imaging/buffer.rs#L34-L177), [`GrayImage`](file:///C:/C/Sem-7/Digital_Image_Processing/omniscan/src/imaging/buffer.rs#L182-L260), and [`BinaryImage`](file:///C:/C/Sem-7/Digital_Image_Processing/omniscan/src/imaging/buffer.rs#L265-L348).
  - Concrete public API without speculative traits:
    - `to_grayscale(buffer: &ImageBuffer) -> Result<GrayImage, ImageError>`
    - `otsu_threshold(gray: &GrayImage) -> Result<BinaryImage, ImageError>`
    - `otsu_threshold_with_level(gray: &GrayImage) -> Result<(BinaryImage, f64), ImageError>`
    - `adaptive_gaussian_threshold(gray: &GrayImage, params: &AdaptiveGaussianParams) -> Result<BinaryImage, ImageError>`
    - `adaptive_gaussian_threshold_default(gray: &GrayImage) -> Result<BinaryImage, ImageError>`
    - `AdaptiveGaussianParams` struct with validated constructor `new(block_size, c)`.

### 2. Grayscale Conversion Details
- **Luminance Formula (ITU-R BT.601):**
  $$Y = 0.299 \times R + 0.587 \times G + 0.114 \times B$$
  Implemented using OpenCV integer fixed-point arithmetic with half-up rounding:
  $$Y = \left\lfloor\frac{299 \times R + 587 \times G + 114 \times B + 500}{1000}\right\rfloor$$
- **Channel Ordering Support:**
  - `PixelFormat::Rgb8`: Processed via `COLOR_RGB2GRAY`. Verified on primary colors: Red (255, 0, 0) -> 76, Green (0, 255, 0) -> 150, Blue (0, 0, 255) -> 29, White (255, 255, 255) -> 255, Black (0, 0, 0) -> 0, Mid-gray (128, 128, 128) -> 128.
  - `PixelFormat::Bgr8`: Processed via `COLOR_BGR2GRAY`. Verified on BGR ordering: Blue (255, 0, 0) -> 29, Red (0, 0, 255) -> 76.
  - `PixelFormat::Gray8`: Passthrough without redundant mathematical operations or allocation overhead.
- Preserves input dimensions (`width`, `height`).

### 3. Otsu Global Thresholding Details
- **Implementation & Binary Polarity:**
  - Applied via `threshold(..., THRESH_BINARY | THRESH_OTSU)`.
  - Polarized in strict adherence to the OMNISCAN domain contract:
    - Pixels $\le T^*$ are mapped to **0** (dark ink / filled pencil mark foreground).
    - Pixels $> T^*$ are mapped to **255** (unmarked white sheet paper background).
- **Threshold Telemetry:**
  - `otsu_threshold_with_level` exposes the computed threshold $T^*$ as `f64` for explanation and validation.
- **Degenerate Histogram Behavior:**
  - Uniform all-white (255): Computed threshold is 0.0; all pixels become 255 (background).
  - Uniform all-black (0): Computed threshold is 0.0; all pixels become 0 (foreground).
  - Uniform gray (128): Computed threshold is 0.0; all pixels become 255 (background).
  - All outputs verified through `BinaryImage::new` ensuring no invalid intermediate values.

### 4. Adaptive Gaussian Thresholding Details
- **Local Neighborhood Thresholding:**
  - Applied via `adaptive_threshold(..., ADAPTIVE_THRESH_GAUSSIAN_C, THRESH_BINARY, block_size, C)`.
  - Evaluates local Gaussian-weighted average minus $C$:
    $$T(x, y) = \text{GaussianMean}(x, y, block\_size) - C$$
    - Pixels $\le T(x, y)$ mapped to **0** (foreground).
    - Pixels $> T(x, y)$ mapped to **255** (background).
- **Parameter Validation & Safeguards:**
  - `block_size`: Validated to be odd, $> 1$, and $\le \min(width, height)$.
  - `c`: Validated to be non-NaN and finite.
  - Invalid parameters return typed `ImageError::InvalidAdaptiveParams { message }` rather than silently clamping.
  - If image dimensions are smaller than `block_size`, an explicit `ImageError::InvalidAdaptiveParams` error is returned.
  - Default parameters: `block_size = 21, c = 10.0`.

### 5. Memory Safety & Resource Safeguards
- Slices from project-owned buffers are transferred directly into local OpenCV `Mat` wrappers and copied into new project-owned `Vec<u8>` buffers.
- Pre-allocation checks on width, height, stride, and maximum allocation cap (`MAX_IMAGE_PIXEL_BYTES = 120 MiB`) remain enforced.
- Zero `unsafe` blocks.

### 6. Validation Results & Quality Gates
- `cargo fmt --check`: **PASSED** (0 formatting diffs).
- `cargo clippy --offline --all-targets -- -D warnings`: **PASSED** (0 warnings across all targets).
- `cargo test --offline --all-targets`: **PASSED** (**101/101 tests passing**, 0 failed, 0 ignored):
  - **59** unit tests in `src/lib.rs` (48 prior + 11 new preprocessing unit tests).
  - **7** integration tests in `tests/preprocessing_tests.rs`.
  - **10** integration tests in `tests/image_decoding_tests.rs`.
  - **8** integration tests in `tests/cli_smoke_tests.rs`.
  - **8** integration tests in `tests/manifest_and_labels_tests.rs`.
  - **9** integration tests in `tests/template_validation_tests.rs`.
- `cargo tree -p opencv`: **PASSED** (only minimal features `imgcodecs` and `imgproc` enabled).

### 7. Milestone M2 Phase Status
- **Current Status:** Phase 2C complete and verified.
- **Next Phase:** Phase 2D (CLI Ingestion/Validation Integration, Diagnostic Threshold Evaluation, and Benchmark Pipeline).


---

## [2026-11-09] — Milestone M2 Phase 2D: CLI Integration, Diagnostic Artifacts & Preprocessing Benchmarks

### 1. Stride Handling Resolution (`src/imaging/preprocessing.rs`)
- **Row-Padded Buffer Invariance:**
  - Resolved potential coordinate drift and memory contamination in `to_grayscale`.
  - When an incoming `ImageBuffer` exhibits a padded row stride (`stride > width * channels`), pixel data is unpadded row-by-row into a continuous pixel slice before transferring to OpenCV's native layout.
  - Guarantees that all downstream `GrayImage` and `BinaryImage` representations maintain packed row layouts (`stride == width`), eliminating padding byte artifacts across binarization, serialization, and future alignment stages.

### 2. CLI `validate-image` Subcommand (`src/main.rs`)
- **Command Syntax:** `omniscan validate-image <IMAGE_PATH>`
- **Telemetry & Validation Output:**
  - Container file size (bytes & KB) vs. fully decoded memory footprint (`width * height * channels`).
  - Spatial dimensions (`width` x `height`), pixel format (`Rgb8`, `Bgr8`, `Gray8`), channel count, and row stride.
  - Explicit limit validation: `MAX_IMAGE_DIMENSION` (15,000 px) and `MAX_IMAGE_PIXEL_BYTES` (120 MiB).
- **Error Handling:**
  - Non-existent files return typed `ImageError::FileNotFound`.
  - Unsupported file extensions (.bmp, .gif, .pdf, etc.) return `ImageError::UnsupportedFormat`.
  - Corrupt, truncated, or unreadable streams return `ImageError::DecodeFailed`.

### 3. CLI `preprocess` Subcommand & Pipeline (`src/imaging/pipeline.rs`, `src/cli.rs`)
- **Command Syntax:** `omniscan preprocess <IMAGE_PATH> --output <DIR> [--method otsu|adaptive|both] [--block-size <ODD_INT>] [--c <FLOAT>] [--benchmark] [--benchmark-iterations <INT>]`
- **Supported Methods:**
  - `otsu`: Computes global Otsu threshold $T^*$ and emits `grayscale.png`, `otsu.png`, and `diagnostics.json`.
  - `adaptive`: Applies local adaptive Gaussian thresholding with validated `block_size` and constant `C`, emitting `grayscale.png`, `adaptive.png`, and `diagnostics.json`.
  - `both` (default): Executes and emits both binarization artifacts alongside `grayscale.png` and `diagnostics.json`.
- **Parameter Validation & Safety:**
  - Rejects even or non-positive `block_size` values before CV processing.
  - Validates `c` against NaN and infinite values.
  - Guard against input overwrites: canonicalizes paths and returns `OmniscanError::ValidationFailed` if an output artifact would overwrite the source input image.

### 4. Output Artifacts & Serialization Boundary (`src/imaging/opencv_adapter.rs`)
- **Encapsulated Image Serialization:**
  - Added `save_gray_image(path, &GrayImage)` and `save_binary_image(path, &BinaryImage)` within the imaging adapter boundary.
  - Uses native OpenCV `imwrite` with `CV_8UC1` under the hood while completely preventing OpenCV `Mat` types from leaking into CLI or pipeline callers.
- **Diagnostic JSON Schema (`diagnostics.json`):**
  - Adheres to schema `"omniscan-preprocessing-diagnostics-v1"`.
  - Records input image metadata (container size, width, height, format, channels, stride, decoded size).
  - Preprocessing configuration (method, calculated Otsu threshold $T^*$, adaptive block size & constant C).
  - Explicit dictionary of generated output artifact paths.
  - High-precision microsecond timing breakdown (`decode_micros`, `grayscale_micros`, `otsu_micros`, `adaptive_micros`, `artifact_write_micros`, `total_pipeline_micros`).
  - Optional `benchmark` telemetry object when benchmark mode is requested.
  - **Atomic Writing:** Diagnostics are written to a temporary sibling file (`.diagnostics.json.tmp.<pid>`) and atomically renamed into place to prevent partial reads or corrupt states during process interruption.

### 5. Preprocessing Benchmark Engine (`src/imaging/diagnostics.rs`)
- **Measurement Methodology:**
  - Isolates pure algorithmic computer vision execution (grayscale conversion and thresholding) from image decoding and disk I/O.
  - Performs 2 warm-up iterations to prime CPU caches, OpenCV lookup tables, and OS threadpools before timing.
  - Runs $N$ measured iterations (default: 10, configurable via `--benchmark-iterations`).
  - Computes throughput in images/second ($N / \text{total\_duration\_seconds}$).
  - Computes comprehensive latency statistics (minimum, median, mean, maximum in microseconds) for total preprocessing time as well as individual breakdown stages (`grayscale_latency_micros`, `otsu_latency_micros`, `adaptive_latency_micros`).
- **Statistical Invariants:**
  - Pure statistical calculation handles even and odd sample sizes, verifying that $min \le median \le max$.

### 6. Validation Results & Quality Gates
- `cargo fmt --check`: **PASSED** (0 formatting diffs).
- `cargo clippy --offline --all-targets -- -D warnings`: **PASSED** (0 warnings across all targets without `--all-features`).
- `cargo test --offline --all-targets`: **PASSED** (**117/117 tests passing**, 0 failed, 0 ignored):
  - **65** unit tests in `src/lib.rs` (59 prior + 4 diagnostics/benchmarking + 2 CLI parsing).
  - **10** integration tests in `tests/cli_preprocessing_tests.rs`.
  - **10** integration tests in `tests/image_decoding_tests.rs`.
  - **8** integration tests in `tests/cli_smoke_tests.rs`.
  - **8** integration tests in `tests/manifest_and_labels_tests.rs`.
  - **7** integration tests in `tests/preprocessing_tests.rs`.
  - **9** integration tests in `tests/template_validation_tests.rs`.
- `cargo tree -p opencv`: **PASSED** (only minimal features `imgcodecs` and `imgproc` enabled).
- Release binary compiled and verified on Windows 11 development environment.

### 7. Milestone M2 Phase Status
- **Current Status:** Phase 2D complete and verified.
- **Milestone M2 Verdict:** **READY FOR M2 FINAL REVIEW**.


---

## [2026-11-09] — Milestone M2: Engineering Audit Remediation & Final Quality Gate Validation

### 1. Timing Telemetry Reconciliation (Finding 1)
- Added `benchmark_micros: Option<u64>` to [`TimingBreakdown`](file:///C:/C/Sem-7/Digital_Image_Processing/omniscan/src/imaging/diagnostics.rs).
- Clarified timing semantics:
  - `total_pipeline_micros` measures end-to-end wall-clock duration of `execute_preprocess`.
  - When `--benchmark` is enabled, the benchmark phase (2 warm-up + $N$ measured runs) is explicitly recorded in `benchmark_micros`.
  - In `BenchmarkSummary`, steady-state latency statistics (`latency_micros`) explicitly exclude warm-up runs.
  - Distinguishes first-run cold-start timing (which may include one-time library/thread-pool startup overhead) from steady-state benchmark iterations.

### 2. Pre-Decode Memory Guard Documentation (Finding 2)
- Explicitly documented decoding and allocation order in [`src/imaging/opencv_adapter.rs`](file:///C:/C/Sem-7/Digital_Image_Processing/omniscan/src/imaging/opencv_adapter.rs):
  - Ingestion occurs in two stages: OpenCV native `imread` decompresses into memory, followed by project-owned dimensional and allocation bounding.
  - Clarified that `MAX_IMAGE_PIXEL_BYTES` (120 MiB) bounds the project-owned `ImageBuffer` and is not a hard process-wide ceiling.
  - Documented decompression bomb risks on highly compressed inputs as an accepted academic MVP limitation.

### 3. Partial Artifact Write Failure Handling (Finding 3)
- Fixed silent write failure bug in `save_raw_gray_png`: verified that OpenCV `imwrite` returns `true`; if it returns `false`, an explicit `ImageError::IoError` is returned instead of silently discarding failure.
- Guaranteed that `diagnostics.json` is written atomically only upon full pipeline success, preventing partial runs from being recorded as successful.
- Documented that unrolled sequential writes may leave earlier PNG artifacts on disk if a subsequent write fails.

### 4. Destination Directory Overwrite Semantics (Finding 4)
- Formally documented in CLI and pipeline doc comments that re-running `preprocess` into an existing output directory overwrites artifact files in-place.
- Preserved strict protection against overwriting original source input files.

### 5. Defensive Non-Contiguous Mat Handling (Finding 5)
- Added explicit continuity verification after `mat.clone()` in `decode_image_with_order`. If a decoded matrix cannot be converted into a continuous row layout, an explicit `ImageError::DecodeFailed` error is returned.

### 6. CI Download Integrity Checksum (Finding 6)
- Pinned and verified authoritative SHA-256 checksum in `.github/workflows/ci.yml`:
  `bff38466091c313dac21a0b73eea8278316a89c1d434c6f0b10697e087670168`
- Added PowerShell `Get-FileHash` verification step that halts CI with an explicit error message on mismatch.
- Updated CI test step to run `cargo test --all-targets`.

### 7. Final Quality Gate Status
- `cargo fmt --check`: **PASSED** (0 formatting diffs).
- `cargo clippy --offline --all-targets -- -D warnings`: **PASSED** (0 compiler warnings).
- `cargo test --offline --all-targets`: **PASSED** (**120/120 tests passing**, 0 failures, 0 ignored).
- `cargo build --release --offline`: **PASSED**.
- `cargo tree -p opencv`: **PASSED** (only minimal `imgcodecs` and `imgproc` features).
- **Remediation Verdict:** **READY FOR COMMIT WITH ACCEPTED LIMITATIONS**.
