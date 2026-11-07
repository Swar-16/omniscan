# OMNISCAN — Agent-Ready Documentation Package

This file is a consolidated reading copy. Prefer the individual files in the repository for agent workflows.


---

<!-- Source: AGENTS.md -->

# AGENTS.md — OMNISCAN Repository Operating Contract

This file is the mandatory instruction set for any AI coding agent working in this repository. Read it before changing files.

## 1. Mission

Build OMNISCAN: a configurable Optical Mark Recognition (OMR) system. The first deliverable is a reliable, explainable Rust CLI academic MVP. The long-term direction is a local-first platform with web and desktop interfaces sharing the same Rust processing core.

## 2. Current phase and priority

The current phase is **Phase 0: research and foundation** unless the repository owner explicitly changes it.

Priority order:
1. Confirm dataset access, license and label schema.
2. Select and document one template for the first end-to-end pipeline.
3. Verify Rust/OpenCV build environment.
4. Establish minimal Rust project, tests and CI.
5. Implement one thin end-to-end vertical slice.
6. Expand algorithms only after the slice works.

Do not start GUI, database, authentication, cloud deployment or ML work while the academic CLI is incomplete.

## 3. Non-negotiable constraints

- Rust is the authoritative implementation language for the academic OMR core.
- Use OpenCV through Rust bindings for computer-vision primitives unless a written ADR changes this.
- No machine learning in the academic MVP.
- The MVP is a CLI, not a web or desktop application.
- The MVP must work without PostgreSQL or network access at runtime.
- Do not invent dataset contents, labels, accuracy, performance measurements or test results.
- Do not claim support for layouts or identifiers that have not been implemented and tested.
- Preserve original scans; derived images are separate artifacts.
- Ambiguous responses must remain explicitly ambiguous/review-required. Never silently guess.
- Do not add dependencies without explaining their purpose and maintenance/build impact.
- Do not download large datasets automatically as part of build/test.
- Do not commit dataset files unless the repository owner has explicitly approved it and licensing/size have been checked.
- Do not add a second database such as SQLite as a shortcut; PostgreSQL is a future-platform direction, while the academic CLI is file-based.
- Do not use unsafe Rust unless justified, isolated and documented.
- Do not make unrelated broad refactors.

## 4. Required workflow for every task

1. Read this file and the relevant documents under `docs/`.
2. Inspect the current code and tests before proposing changes.
3. State the intended change, affected modules, risks and acceptance criteria.
4. Implement the smallest coherent increment.
5. Add or update tests.
6. Run applicable checks and report exact commands/results. If a command could not run, say why.
7. Update documentation when behavior, configuration, output or assumptions change.
8. Summarize changed files, decisions, known limitations and next task.
9. Do not mark a task complete if its acceptance criteria are unmet.

## 5. Git behavior

- Never force-push, rewrite shared history, delete branches, or push without explicit user permission.
- Work on a task-specific branch when the environment supports Git branches.
- Prefer focused Conventional Commits, e.g. `feat(imaging): add validated image loading`.
- Never commit secrets, personal data, licensed data without permission, build artifacts, or large generated images.
- Do not create a commit unless requested or the agent environment's task explicitly authorizes commits.

## 6. Rust quality gate

Run, when applicable:
- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test`

For Windows/OpenCV build problems, report the exact toolchain, environment variables and error. Do not silently replace OpenCV with a different stack.

## 7. Architecture rules

- Keep domain types and scoring independent of OpenCV.
- Confine OpenCV `Mat` and binding-specific details to imaging adapters where practical.
- Keep `evaluation` independent of image recognition.
- Keep template geometry and recognition thresholds in validated configuration, not scattered magic constants.
- Use typed errors rather than panics for expected input failures.
- Bound memory and concurrency for batch work.
- Make output reproducible for identical inputs, configuration and software version where practical.
- Version templates, answer keys and processing configuration.
- Use `HashMap` for candidate-code lookup and `Vec` for ordered question responses unless measurements justify another structure.
- Don't introduce elaborate abstractions without a concrete use case.

## 8. Definition of done

A task is done only when:
- acceptance criteria are met;
- tests are added or updated and pass, or failures are explicitly explained;
- errors and invalid inputs are handled;
- docs and examples match actual behavior;
- no unsupported claims are made;
- limitations are listed;
- the final response gives commands run and their results.

## 9. Stop and ask the owner

Ask before:
- changing an accepted ADR;
- adding ML to the academic MVP;
- adding a database dependency to the academic CLI;
- committing or redistributing a dataset;
- changing public JSON/CSV output schemas incompatibly;
- deleting or rewriting user data;
- making a major scope or licensing decision.

## 10. Agent response format

For each task, report:
1. **Implemented**
2. **Design decisions**
3. **Files changed**
4. **Validation commands and results**
5. **Known limitations**
6. **Next smallest task**


---

<!-- Source: OMNISCAN_COMPLETE_DOCS.md -->

# OMNISCAN — Complete Documentation Bundle

> Consolidated copy of all project Markdown documentation. Individual files are also included in the ZIP archive.


---

<!-- Source: adr/ADR-001-rust-core.md -->

# ADR-001: Rust as the Primary Processing Core

- **Status:** Accepted

## Context
The academic assignment needs an efficient, maintainable CLI and the long-term platform should reuse its processing engine across interfaces.

## Decision
Implement the core OMR pipeline in Rust.

## Consequences
- Strong types and explicit error handling support maintainability.
- Rust supports efficient CPU-bound processing and controlled concurrency.
- The team must invest in Rust and OpenCV binding setup.
- Python may remain useful for dataset exploration and optional ML experimentation, but it is not the authoritative MVP engine.


---

<!-- Source: adr/ADR-002-opencv.md -->

# ADR-002: OpenCV Through Rust Bindings

- **Status:** Accepted

## Context
OMR processing requires image decoding, filtering, thresholding, contours and geometric transformations.

## Decision
Use OpenCV through Rust bindings for the academic CV pipeline.

## Consequences
- Leverage established CV primitives instead of reimplementing foundational image operations.
- Keep OpenCV types behind adapters and project-owned image abstractions.
- Pin and document the native OpenCV/toolchain setup for reproducibility.
- Benchmark alternative algorithms rather than assuming defaults are optimal.


---

<!-- Source: adr/ADR-003-no-ml-mvp.md -->

# ADR-003: No Machine Learning in the Academic MVP

- **Status:** Accepted

## Context
The assignment deadline is approximately 1–2 weeks and the objective is to demonstrate algorithmic image processing with explainable behavior.

## Decision
Do not use ML for the academic MVP. Use deterministic image processing and template-based recognition.

## Consequences
- Easier to explain, debug and benchmark.
- Scope is bounded to documented templates and supported scan conditions.
- Difficult or ambiguous cases must be flagged rather than guessed.
- Optional ML can be evaluated in a later subsystem without replacing the deterministic baseline.


---

<!-- Source: adr/ADR-004-modular-monolith.md -->

# ADR-004: Start With a Modular Monolith

- **Status:** Accepted

## Context
The product is early-stage and the academic deliverable is a single CLI. Distributed services would add operational complexity before there is evidence they are needed.

## Decision
Use a modular monolith with clear module boundaries.

## Consequences
- Easier local development, testing and deployment.
- Modules can later be extracted if scale or independent deployment justifies it.
- Enforce dependency direction and avoid cross-module implementation leakage.


---

<!-- Source: adr/ADR-005-shared-core.md -->

# ADR-005: Share One Processing Core Across Interfaces

- **Status:** Accepted

## Context
The roadmap includes CLI, desktop and web experiences. Duplicating OMR logic would create inconsistent results and multiply maintenance.

## Decision
Keep recognition, candidate resolution and evaluation in a shared Rust core. Interfaces call into that core through suitable application boundaries.

## Consequences
- Consistent behavior across CLI and GUI.
- Core APIs must not depend on a specific UI.
- Web deployment may require a service/API boundary; desktop may call a local library or local service.


---

<!-- Source: adr/ADR-006-postgresql.md -->

# ADR-006: PostgreSQL for Future Platform Persistence

- **Status:** Accepted direction; deployment lifecycle open

## Context
The long-term product needs relational integrity for projects, templates, answer keys, jobs, candidates, results and audit history. PostgreSQL is the chosen database.

## Decision
Use PostgreSQL as the canonical structured application database for the platform. The academic CLI remains file-based and does not require a database.

## Consequences
- Use migrations and explicit schema versioning.
- Store metadata and structured records in PostgreSQL.
- Store large image and report artifacts in filesystem/object storage and keep references in the database.
- Local-first deployment needs a deliberate PostgreSQL installation, startup, backup and upgrade strategy. This must be validated before implementation.
- Do not add SQLite solely as a shortcut without revisiting the storage decision explicitly.


---

<!-- Source: adr/ADR-007-hybrid-layout.md -->

# ADR-007: Hybrid OMR Layout Strategy

- **Status:** Accepted

## Context
Different institutions use different sheet layouts. Fully hard-coded coordinates do not scale, while universal automatic layout inference is too risky for the initial product.

## Decision
Support known-template processing first, assisted template configuration next, and automatic layout discovery as a later capability.

## Consequences
- Initial implementation is deterministic and bounded.
- Template geometry is explicit and versioned.
- Unfamiliar layouts can be assisted rather than silently misread.
- “Any page size” is handled through geometric normalization, not assumed to mean arbitrary layout understanding.


---

<!-- Source: adr/ADR-008-confidence-review.md -->

# ADR-008: Confidence-Aware Recognition

- **Status:** Accepted

## Context
A wrong candidate ID or response can cause an incorrect result. A forced prediction hides uncertainty.

## Decision
Recognition must support explicit ambiguous and review-required states. Confidence/evidence should be retained where meaningful.

## Consequences
- Thresholds and ambiguity margins need calibration.
- Reports can distinguish automatically accepted from unresolved results.
- Human review becomes a first-class future workflow.
- The system should prefer abstention over an unsupported guess.


---

<!-- Source: adr/ADR-009-preserve-originals.md -->

# ADR-009: Preserve Original Source Documents

- **Status:** Accepted

## Context
Preprocessing and alignment may modify image appearance. The original scan is essential for audit, debugging and reprocessing.

## Decision
Never overwrite original scans. Derived images and annotations are separate artifacts.

## Consequences
- Additional storage is required.
- Every derived artifact should be traceable to its source and processing configuration.
- Reprocessing remains possible when algorithms or templates improve.


---

<!-- Source: adr/ADR-010-versioned-config.md -->

# ADR-010: Version Templates and Processing Configurations

- **Status:** Accepted

## Context
A result is only reproducible if the template and processing parameters used to generate it are known.

## Decision
Version templates, answer keys and processing configurations. Record the applicable version or checksum with each processing run.

## Consequences
- Configuration changes do not silently alter historical results.
- Benchmark results can be tied to exact configurations.
- The platform needs migration and compatibility policies for old configurations.


---

<!-- Source: adr/README.md -->

# Architecture Decision Records

ADRs record important technical choices and their status. A decision may be revisited when evidence changes.

| ID | Decision | Status |
|---|---|---|
| ADR-001 | Rust as primary processing core | Accepted |
| ADR-002 | OpenCV through Rust bindings | Accepted |
| ADR-003 | No ML in academic MVP | Accepted |
| ADR-004 | Modular monolith initially | Accepted |
| ADR-005 | Shared processing core for CLI/web/desktop | Accepted |
| ADR-006 | PostgreSQL for future platform persistence | Accepted direction; lifecycle details open |
| ADR-007 | Hybrid template strategy: known + assisted + future discovery | Accepted |
| ADR-008 | Confidence-aware recognition and human review | Accepted |
| ADR-009 | Preserve original source documents | Accepted |
| ADR-010 | Version templates and processing configuration | Accepted |

See each ADR for context, decision, consequences and open questions.


---

<!-- Source: algorithm-design.md -->

# Algorithm Design

## 1. Pipeline overview

```text
Input scan
   |
Decode + validate
   |
Quality checks
   |
Grayscale / denoise / threshold
   |
Registration detection + geometric alignment
   |
Template coordinate transform
   |
ROI extraction
   |
Bubble fill measurement
   |
Response-state classification
   |
Candidate identifier decoding + mapping
   |
Answer-key evaluation
   |
JSON / CSV + annotated diagnostics
```

## 2. Image preprocessing

Investigate grayscale conversion, denoising and thresholding. Compare Otsu global thresholding with adaptive thresholding on the selected dataset. Do not assume one method is universally superior; record results by scan condition.

Keep the original image unchanged. Every derived image should be a separate processing artifact.

## 3. Geometric alignment

If the template has registration markers or stable page features, detect them and estimate a geometric transform. Use an affine transform for rotation/scale/translation when adequate; use a homography when perspective distortion requires a projective mapping and enough reliable correspondences are available.

Reject or flag poor alignment rather than extracting ROIs from a badly registered page.

## 4. Template and normalized coordinates

Represent regions relative to a reference page coordinate system. A template should define:
- reference width and height;
- page orientation;
- registration features;
- candidate identifier region;
- question and option regions;
- question type and option count;
- any calibration or tolerance parameters.

Normalized coordinates help handle different raster resolutions. They do not automatically solve unknown layout recognition.

## 5. Bubble fill measurement

For each expected bubble ROI, compute a darkness or foreground-density measure. One simple metric is:

\[
d = \frac{\text{foreground pixels inside bubble mask}}
         {\text{total pixels inside bubble mask}}
\]

A circular mask is preferable when bubble geometry is known. A rectangular ROI may be used for early experiments, but it can include border and neighboring artifacts.

An integral image can accelerate rectangular sum queries: build it in \(O(W H)\), then query each axis-aligned rectangle in \(O(1)\). For circular regions, use precomputed masks or another measured approach.

## 6. Response classification

Use calibrated thresholds and explicit margins between the strongest candidate and other candidates. A question can be:
- `Selected(option)`;
- `Blank`;
- `Multiple(options)`;
- `Ambiguous(reason)`.

Do not force a low-confidence bubble into a selected state. Thresholds must be calibrated on training/validation samples and evaluated on held-out scans.

## 7. Candidate identification

Decode the identifier using the method supported by the chosen template. For bubble-coded digits, evaluate each digit column/row using the same fill analysis, with explicit handling for blank or multiple marks. Resolve the decoded code through a mapping table.

Use a hash map for candidate-code lookup: expected \(O(1)\) lookup. Detect duplicate codes before processing.

## 8. Evaluation

For each question, compare the recognized response with the answer key and apply the configured marking policy. Define behavior for:
- correct response;
- incorrect response;
- blank response;
- multiple marks;
- ambiguous response;
- multiple-select response;
- partial credit.

The evaluation module should be independent of image processing and tested using synthetic domain objects.

## 9. Data structures and complexity

| Need | Candidate structure | Rationale |
|---|---|---|
| Candidate code lookup | `HashMap` | Expected constant-time lookup |
| Ordered question responses | `Vec` | Contiguous storage and indexed access |
| Duplicate detection | `HashSet` | Expected constant-time membership |
| Irregular spatial lookup | Spatial grid or R-tree | Add only if geometry/query volume justifies it |
| Batch scheduling | Bounded queue | Controls peak memory |
| Per-question scoring | Sequential/vector iteration | \(O(Q)\) per sheet |

Avoid introducing complex data structures merely to make the project appear sophisticated. Measure actual bottlenecks first.

## 10. Duplicate scans

Use cryptographic hashes to identify byte-identical files. Perceptual hashes can flag visually similar scans, but must not automatically discard them: different candidates can have near-identical blank forms. Treat similarity as a review signal.


---

<!-- Source: architecture.md -->

# System Architecture

## 1. Architectural direction

Use a **modular monolith** initially. The Rust processing core is shared by the CLI and future application interfaces. Avoid microservices until real deployment or scaling requirements justify them.

## 2. Target architecture

```text
                    User Interfaces
             +----------+----------+
             |          |          |
            CLI       Desktop      Web
                        |           |
                     Tauri       React/TS
                        \           /
                         API / App Layer
                               |
                      Shared Rust Core
       +-----------------------+-----------------------+
       |                       |                       |
   Ingestion               OMR Engine              Evaluation
       |                       |                       |
   Validation       Preprocess -> Align -> ROI     Answer Key
   File handling    Layout -> Bubble recognition  Scoring
                    Candidate identification       Results
       +-----------------------+-----------------------+
                               |
                Persistence / Artifact Storage
                 PostgreSQL + File/Object Storage
                               |
                 Optional Intelligence Subsystem
                  Python ML experiments/services
```

The academic MVP implements the Rust core and CLI path only.

## 3. Core modules

- `core`: shared domain types, errors, configuration and traits.
- `imaging`: loading, preprocessing, thresholding and geometry.
- `layout`: templates, regions and registration features.
- `recognition`: bubble analysis, identifier extraction and confidence.
- `evaluation`: answer keys, candidate mapping and scoring.
- `pipeline`: orchestration of the processing stages.
- `output`: JSON/CSV serialization and annotated images.
- `benchmark`: metrics and experiment reporting.
- `cli`: argument parsing and command handlers.

## 4. Dependency direction

The domain/core layer should not depend on OpenCV. OpenCV-specific types should be confined to imaging adapters. Recognition and evaluation should consume project-owned types rather than exposing `opencv::core::Mat` throughout the codebase.

Example abstraction:

```rust
pub trait ImagePreprocessor {
    fn preprocess(
        &self,
        input: &ImageFrame,
    ) -> Result<ProcessedImage, ProcessingError>;
}

pub trait BubbleRecognizer {
    fn recognize(
        &self,
        image: &ProcessedImage,
        layout: &OmrLayout,
    ) -> Result<Vec<QuestionResponse>, RecognitionError>;
}
```

These interfaces are illustrative; refine them after the selected dataset and template are known.

## 5. Storage boundaries in the future platform

PostgreSQL is the canonical store for structured application data: projects, templates, answer-key versions, jobs, candidate mappings, results, review actions and audit events.

Store large binary artifacts—original scans, normalized images, annotations and generated reports—in filesystem or object storage. Store stable references and metadata in PostgreSQL rather than placing all image bytes in relational rows.

The local-first PostgreSQL lifecycle remains an ADR to validate. The academic CLI should not depend on a database.

## 6. Concurrency and memory

Use bounded work queues for batch processing. Parallelize independent page-level work only after measuring memory per in-flight image. A configurable worker count should be available. Do not read an unbounded batch into RAM.

## 7. Future ML boundary

Keep ML optional and isolated behind a stable interface. Python may be used for training and experimentation; the deterministic Rust pipeline remains the default. Any ML-assisted output must expose confidence and preserve human review for uncertain cases.


---

<!-- Source: benchmark-methodology.md -->

# Benchmark and Evaluation Methodology

## 1. Purpose

Measure recognition quality and computational performance using a reproducible protocol. Report actual results only after running the implementation.

## 2. Dataset partitioning

Separate data used for threshold calibration from final evaluation. Where possible, split by physical sheet or source session rather than randomly splitting near-duplicate crops. Keep a manifest of all samples and ground-truth labels.

## 3. Recognition metrics

Report at least:
- per-bubble classification precision, recall and F1;
- per-question exact response accuracy;
- per-sheet exact response accuracy;
- blank detection accuracy;
- multiple-mark detection accuracy;
- candidate-ID exact match rate;
- unresolved/ambiguous rate.

For multi-class or imbalanced labels, include a confusion matrix and macro-F1. Explain the unit of evaluation (bubble, question or sheet).

## 4. Operational metrics

Measure:
- end-to-end latency per sheet;
- stage-level latency;
- throughput in sheets/second;
- peak resident memory;
- batch completion time;
- failure rate.

Report warm-up policy, number of repetitions, dataset size, worker count, build profile and hardware/software environment.

## 5. Experimental comparisons

Useful experiments include:
- Otsu vs adaptive thresholding;
- denoising methods;
- alignment on vs off (where relevant);
- different fill thresholds;
- ROI mask variants;
- sequential vs bounded parallel batch execution.

Change one factor at a time where possible. Avoid selecting parameters on the final test set.

## 6. Reporting format

Every result table should state:
- dataset and split;
- sample count;
- configuration identifier;
- metric definition;
- measured value;
- test environment.

Include representative successes and failure cases with annotated images. Explain failure causes rather than reporting only an aggregate score.

## 7. Reproducibility

Record Git commit, Rust version, dependency versions, configuration checksum, dataset manifest checksum and benchmark command. Do not invent or estimate measurements.


---

<!-- Source: cli-specification.md -->

# CLI Specification (Proposed)

The exact command names may change during implementation. Keep the CLI discoverable, composable and script-friendly.

## 1. Proposed commands

```text
omniscan
  validate-image <IMAGE>
  inspect-template <TEMPLATE.json>
  process <IMAGE> --template <TEMPLATE.json> --answer-key <KEY.json>
  batch <INPUT_DIR> --template <TEMPLATE.json> --answer-key <KEY.json>
  evaluate <RESPONSES.json> --answer-key <KEY.json>
  benchmark <MANIFEST.json> --template <TEMPLATE.json>
```

## 2. Output conventions

- Machine-readable JSON for structured results.
- CSV for tabular batch results.
- Diagnostic images written to an explicit output directory.
- Progress and human-readable diagnostics to stderr.
- Avoid mixing progress text into JSON stdout.
- Return non-zero exit codes for invalid input, configuration errors and processing failures.

## 3. Configuration

Keep template, preprocessing and recognition configuration separate where useful:

```text
configs/
  templates/
    initial-template.json
  preprocessing.json
  recognition.json
```

Every run should record the configuration version or a checksum so results can be reproduced.

## 4. Error handling

Errors should identify the stage and affected file. Distinguish:
- invalid or unsupported file;
- image decode failure;
- invalid template;
- alignment failure;
- recognition ambiguity;
- candidate mapping failure;
- answer-key validation failure;
- output write failure.

Ambiguity is not necessarily a fatal process error: it may produce a result requiring review.


---

<!-- Source: data-model.md -->

# Conceptual Data Model

This document describes the future platform's conceptual entities. The academic CLI may represent these as in-memory Rust structs and JSON configuration rather than database tables.

## 1. Entities

### Project
An exam/evaluation workspace. Has an identifier, name, description, status and timestamps.

### Template
A versioned description of page geometry, registration features, candidate-ID regions, question regions, option locations and supported response types.

### AnswerKey
A versioned set of expected answers and scoring rules associated with an exam/project.

### Candidate
A person or exam participant. Contains an internal identifier and permitted identifying attributes.

### CandidateCodeMapping
Maps a code read from the sheet to exactly one candidate/roll number. Enforce uniqueness where required.

### Scan
A source document and its metadata: file reference, checksum, dimensions, upload time, detected quality and processing status.

### ProcessingJob
Tracks a batch or single-scan run, configuration version, status, timestamps, progress and errors.

### RecognitionResult
Stores candidate identification, per-question response states, confidence/evidence and processing provenance.

### EvaluationResult
Stores awarded marks, maximum marks, question-level breakdown and scoring-policy version.

### ReviewAction
Records a human correction or confirmation, actor, timestamp, old value, new value and reason.

### Artifact
References original scans, aligned images, overlays, annotated images, exports and reports.

## 2. Relationship sketch

```text
Project 1---* TemplateVersion
Project 1---* AnswerKeyVersion
Project 1---* Scan
Project 1---* ProcessingJob
Project 1---* Candidate
Candidate 1---* CandidateCodeMapping
Scan 1---* RecognitionResult
ProcessingJob 1---* RecognitionResult
RecognitionResult 1---1 EvaluationResult
RecognitionResult 1---* ReviewAction
Scan 1---* Artifact
```

## 3. Persistence principles

- PostgreSQL stores structured metadata and transactional state.
- Binary image/report artifacts live in filesystem or object storage.
- Store checksums and artifact references for integrity and traceability.
- Version templates, answer keys and processing configurations.
- Preserve the exact configuration and software version used for each processing run.
- Do not overwrite original scans during preprocessing.


---

<!-- Source: dataset-research-plan.md -->

# Dataset Research and Selection Plan

**Status: Pending research.** No dataset is selected in this document.

## 1. Goal

Choose a public scanned OMR dataset suitable for a short, measurable Rust CLI assignment. Dataset selection determines the initial template, ground-truth format, recognition approach and evaluation claims.

## 2. Minimum comparison set

Investigate at least three candidate public datasets or reproducible research resources. For each, record:

| Criterion | Questions |
|---|---|
| Access | Is there a direct download or a documented request process? |
| License | Is academic use permitted? Are redistribution restrictions clear? |
| Images | Are actual scanned/photo OMR sheets available? |
| Ground truth | Are marked answers labelled per question? |
| Template | Is the sheet geometry documented or inferable? |
| Candidate ID | Is an identifier present and labelled? |
| Scale | How many sheets and pages are available? |
| Variation | Does it include rotation, skew, lighting, noise and print variation? |
| Format | Image/PDF format, resolution and color depth? |
| Reproducibility | Are scripts, paper, annotation schema or benchmark splits available? |

## 3. Selection principles

Prefer a dataset that:
1. can be accessed immediately;
2. includes real scanned sheets;
3. has trustworthy answer-level ground truth;
4. uses a consistent, documented template;
5. is small enough to inspect and process within the deadline;
6. has clear academic usage terms.

A dataset without candidate identifiers can still support answer recognition. In that case, use synthetic sheets for candidate-ID and roll-mapping tests and disclose the split between real and synthetic evaluation.

## 4. Synthetic dataset strategy

Build a generator only after documenting the selected template. Generate known answer patterns and candidate codes, then render bubbles and introduce controlled variations:
- translation, rotation and mild perspective;
- scale and resolution changes;
- blur and sensor noise;
- brightness/contrast changes;
- faint and heavy marks;
- erasures and stray marks;
- multiple marks and blank responses.

Store ground truth beside each generated image in a machine-readable manifest. Use deterministic random seeds. Keep training/tuning samples separate from held-out evaluation samples.

Synthetic data is useful for controlled tests, but it must not be presented as a substitute for real-world validation.

## 5. Deliverables

- Dataset comparison table with URLs, access date and license notes.
- Chosen dataset and rationale.
- Sample inventory and annotation audit.
- Template geometry document.
- Ground-truth schema.
- Synthetic generation specification.
- Train/validation/test split manifest.

## 6. Important limitation

Do not claim that a dataset supports candidate identification, multiple layouts or arbitrary page sizes unless its actual files and labels demonstrate those properties.


---

<!-- Source: engineering-workflow.md -->

# Engineering Workflow and Quality Standards

## 1. Repository principles

- Keep modules cohesive and dependencies directed inward.
- Use explicit domain types instead of passing loosely structured values.
- Keep OpenCV-specific types behind adapters.
- Keep configuration external and validated.
- Preserve original input files.
- Prefer small, reviewable commits.

## 2. Suggested repository structure

```text
omniscan/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── configs/
├── datasets/
│   ├── public/
│   ├── synthetic/
│   ├── answer_keys/
│   └── mappings/
├── docs/
├── src/
│   ├── main.rs
│   ├── cli/
│   ├── core/
│   ├── imaging/
│   ├── layout/
│   ├── recognition/
│   ├── evaluation/
│   ├── pipeline/
│   ├── output/
│   └── benchmark/
├── tests/
└── scripts/
```

Create files incrementally as implementation requires; the tree is a target organization, not a requirement to create empty modules immediately.

## 3. Rust quality

- Run `cargo fmt --check`.
- Run `cargo clippy --all-targets --all-features -- -D warnings` where practical.
- Run `cargo test`.
- Document public types and non-obvious algorithms.
- Avoid unnecessary cloning and unbounded allocations.
- Use `Result` for recoverable errors; avoid panics on user input.
- Validate all external configuration and file paths.

## 4. Git workflow

Use focused conventional commits, for example:

```text
docs: establish OMNISCAN product and engineering baseline
chore: initialize Rust workspace and CI checks
feat(imaging): add validated image loading
feat(layout): add normalized template geometry
feat(recognition): classify bubble response states
feat(evaluation): implement configurable scoring
test(pipeline): add end-to-end OMR fixtures
perf(batch): add bounded parallel processing
```

## 5. Definition of done

A change is done when:
- behavior and assumptions are documented;
- relevant tests pass;
- invalid inputs and failure modes are handled;
- output is deterministic where expected;
- performance-sensitive changes have measurements;
- no original input is overwritten;
- logs and diagnostics are useful;
- the change is committed with a focused message.

## 6. CI/CD direction

For the academic MVP, CI should build and test the Rust project and run formatting/lint checks. Later platform phases can add frontend checks, integration environments, security scanning, packaging and deployment workflows.


---

<!-- Source: mvp-scope.md -->

# Academic MVP Scope

## 1. Objective

Deliver a reliable, explainable Rust CLI that demonstrates an end-to-end OMR pipeline for at least one documented sheet template.

## 2. Included

- Public scanned OMR dataset investigation and selection.
- Synthetic OMR data generation with known ground truth.
- One documented initial template.
- Image loading and validation.
- Grayscale conversion and denoising experiments.
- Otsu and adaptive thresholding comparison.
- Registration marker detection, where the selected template provides suitable markers.
- Alignment and perspective correction where justified.
- Template-based region-of-interest (ROI) extraction.
- Bubble fill measurement and classification.
- Blank, multiple-mark and ambiguous response handling.
- Candidate identifier extraction for the selected template.
- Candidate-to-roll mapping.
- JSON answer key.
- Configurable positive/negative/partial scoring.
- CSV and JSON output.
- Annotated diagnostic images.
- Unit, integration and end-to-end tests.
- Accuracy, latency, throughput and memory measurements.

## 3. Excluded

- Web UI, desktop UI and Tauri packaging.
- PostgreSQL and application persistence.
- Authentication and multi-user support.
- Cloud services.
- ML models and neural inference.
- Automatic understanding of arbitrary unknown layouts.
- Full-featured PDF ingestion unless the chosen dataset makes it essential.

## 4. MVP workflow

1. Read CLI arguments and configuration.
2. Load and validate the input scan.
3. Preprocess the image.
4. Detect registration features and align the sheet if needed.
5. Load the selected template.
6. Extract identifier and response ROIs.
7. Measure bubble fill.
8. Classify each response, retaining ambiguity.
9. Resolve candidate identifier to a roll number.
10. Load answer key and scoring policy.
11. Evaluate responses.
12. Write machine-readable results and annotated diagnostics.

## 5. Acceptance criteria

- A documented template and its coordinate conventions are checked into the repository.
- The CLI rejects invalid input with actionable errors.
- A valid sample completes the full pipeline.
- Each question produces a defined state: selected, blank, multiple or ambiguous.
- Candidate mapping failures are explicitly represented.
- Scoring edge cases are covered by tests.
- Output is deterministic for the same input, template and configuration.
- Ground-truth evaluation reports per-question and per-sheet metrics.
- Benchmarks record machine, build mode, dataset size and configuration.
- Original source scans remain unchanged.

## 6. Scope control

If schedule pressure arises, reduce the number of supported templates and input formats before weakening correctness, tests or evaluation methodology. A narrow, measured implementation is preferable to a broad but unvalidated demo.


---

<!-- Source: product-requirements.md -->

# Product Requirements Document (PRD)

**Product:** OMNISCAN  
**Version:** 0.1  
**Status:** Draft baseline

## 1. Executive summary

OMNISCAN is an OMR (Optical Mark Recognition) recognition, evaluation and analytics platform. It processes scanned examination sheets, identifies candidates, extracts marked responses, evaluates them against an answer key, and produces structured results and reports.

The long-term product is local-first and supports both web and desktop interfaces over a shared Rust processing core. The academic assignment is deliberately narrower: a Rust CLI that demonstrates a well-engineered, measurable OMR pipeline without machine learning.

## 2. Problem statement

Manual checking of objective examination sheets is repetitive and can be slow and error-prone. An OMR workflow must reliably:

- locate and interpret response bubbles;
- distinguish blank, marked and ambiguous responses;
- identify the candidate;
- apply the correct answer key and marking policy;
- retain enough evidence to audit and correct uncertain cases.

A useful system must handle image quality variation and configurable sheet layouts without pretending that every possible form can be understood automatically.

## 3. Product vision

Build a configurable OMR platform that can process known sheet templates deterministically, assist users in configuring unfamiliar templates, and progressively add automatic layout discovery and optional ML assistance.

### Recognition modes

1. **Known Template Mode:** process a configured layout using deterministic geometry and image analysis.
2. **Assisted Template Mode:** propose detected regions and let an operator confirm or correct them.
3. **Automatic Discovery Mode:** infer candidate layout regions for a previously unseen form; uncertain detections require review.

Universal recognition is not a promise of perfect recognition for arbitrary sheets. Page-size independence is achieved through normalized coordinates and geometric transformations; it does not imply layout independence.

## 4. Target users

- Examination administrators and institutions.
- Teachers and evaluators handling objective tests.
- Small organizations conducting assessments.
- Developers integrating OMR processing into larger systems.

The first release is single-user. The architecture should not block future multi-user operation.

## 5. Product goals

- Correctly interpret supported OMR templates under documented conditions.
- Make recognition uncertainty visible and reviewable.
- Support candidate identification and explicit candidate-to-roll mapping.
- Provide configurable scoring rules.
- Process batches with bounded memory use.
- Produce reproducible results and useful diagnostic artifacts.
- Reuse the same processing engine in CLI, desktop and web products.

## 6. Functional requirements

### 6.1 Projects and exams

A project represents an exam or evaluation run and contains its template, answer key, candidate mapping, source scans, processing configuration, jobs, results and reports.

### 6.2 Document ingestion

Long-term platform inputs may include PNG, JPG/JPEG, TIFF and PDF. Support single files, batches and ZIP archives. Validate file type, dimensions, readability and basic image quality before processing.

The academic MVP may initially support a narrower set of raster image formats, depending on dataset availability and time.

### 6.3 Layout and template configuration

A template describes page geometry, registration markers, candidate identifier regions, question regions, option positions, sections and supported response types.

Use normalized coordinates where possible so that a template can be applied across different scan resolutions. Apply alignment or perspective correction before extracting regions when required.

### 6.4 Candidate identification

The long-term platform should support:
- bubble-coded numeric identifiers;
- QR codes;
- barcodes;
- printed text where feasible.

The academic MVP should implement the identifier method actually present in the selected dataset/template. Synthetic samples can provide controlled bubble-coded identifiers and roll mappings if the public dataset lacks them.

Candidate identifiers must resolve uniquely. Unknown or duplicate mappings are review conditions, not reasons to guess.

### 6.5 Response recognition

For each question, the system should determine:
- selected option or options;
- blank response;
- multiple-mark response;
- ambiguous response;
- confidence or evidence score;
- diagnostic region information where available.

Question types include single-choice and multiple-select, with configurable option counts in the long-term platform.

### 6.6 Answer keys

The long-term product should support JSON, CSV, image-assisted extraction with human verification, and a manual editor. The academic MVP starts with a documented JSON schema.

### 6.7 Evaluation

Support configurable positive, negative and partial marking. Define explicit policies for blank answers, multiple marks, invalid responses and unanswered questions. Scoring must be deterministic and independently unit-tested.

### 6.8 Results and reporting

Produce structured JSON and CSV results. The future platform may add PDF scorecards, batch analytics, section summaries, review queues and project dashboards.

### 6.9 Human review

The full platform should show the original and aligned image, relevant response region, detected marks, confidence and extracted result. Corrections should be auditable and trigger recalculation.

## 7. Non-functional requirements

- **Correctness:** no silent acceptance of unresolved ambiguity.
- **Performance:** measure per-page latency, throughput and peak memory.
- **Scalability:** bounded batch processing; avoid loading an entire batch into memory.
- **Reliability:** clear errors, safe failure and reproducible processing.
- **Maintainability:** modular code, explicit types and documented interfaces.
- **Portability:** Windows development support for the academic implementation; broader OS support is a later goal.
- **Security:** validate untrusted uploads, avoid unsafe path handling, and isolate project data in the platform.
- **Observability:** stage-level progress, structured errors and diagnostic outputs.

## 8. Constraints

- Academic assignment deadline: approximately 1–2 weeks.
- Academic submission: Rust CLI only.
- Initial academic implementation: no ML.
- Computer vision: Rust with OpenCV bindings.
- Long-term application database preference: PostgreSQL.
- Initial user model: single user, with future multi-user architecture in mind.

## 9. Success criteria

The academic MVP is successful when it can process a documented supported template end-to-end, identify candidates according to the implemented identifier method, extract responses, score them against a supplied answer key, emit results and diagnostics, and report measured accuracy and performance against ground truth.

No numerical accuracy target is asserted until the dataset and evaluation protocol are finalized.

## 10. Out of scope for the academic MVP

GUI, web and desktop clients; PostgreSQL integration; authentication; multi-user access; cloud deployment; ML inference/training; arbitrary-layout universal recognition; and production-grade PDF workflow.


---

<!-- Source: roadmap.md -->

# Product Roadmap

## Phase 0 — Research and technical foundation

1. Compare at least three public OMR datasets.
2. Select an accessible dataset and inspect annotations.
3. Choose and document one initial template.
4. Define synthetic data generation and ground-truth format.
5. Finalize MVP assumptions and acceptance criteria.
6. Validate Rust/OpenCV toolchain on the development machine.
7. Establish baseline project structure and CI.
8. Finalize benchmark methodology.

**Exit condition:** a dataset, template, ground-truth protocol and build environment are selected.

## Phase 1 — Academic Rust CLI (1–2 weeks)

- Build image loading and preprocessing.
- Implement thresholding experiments.
- Implement registration/alignment as required by the template.
- Extract template-based ROIs.
- Implement bubble fill measurement and classification.
- Decode candidate identifiers for the chosen template.
- Implement candidate mapping and scoring.
- Add JSON/CSV outputs and diagnostic overlays.
- Test and benchmark the end-to-end pipeline.
- Write the academic report with measured results.

**Exit condition:** reproducible CLI demonstration and evidence-based report.

## Phase 2 — Core hardening

- Stabilize domain types and interfaces.
- Improve error taxonomy and configuration validation.
- Add more templates and input formats.
- Add robust batch processing and bounded concurrency.
- Add versioned processing configurations and regression fixtures.

## Phase 3 — Local-first platform

- Add PostgreSQL-backed project, template, answer-key, job and result metadata.
- Add filesystem artifact management.
- Build a Rust application/API layer.
- Add job progress and review workflows.
- Add React/TypeScript web UI and Tauri desktop shell over the shared core.

## Phase 4 — Advanced capabilities

- Assisted template configuration.
- QR/barcode and additional candidate-ID methods.
- PDF and bulk archive workflows.
- Rich analytics and PDF reports.
- Optional ML subsystem for difficult layouts and ambiguous marks.

## Phase 5 — Optional cloud deployment and multi-user

- Authentication and authorization.
- Tenant/project isolation.
- Cloud object storage and managed PostgreSQL.
- Deployment automation, monitoring, backups and operational controls.

## Schedule protection

The academic assignment takes priority. Do not start GUI, database, ML or cloud work until the CLI has a tested end-to-end path.


---

<!-- Source: testing-strategy.md -->

# Testing Strategy

## 1. Testing layers

### Unit tests
- coordinate normalization and transform calculations;
- template validation;
- threshold and fill-density functions;
- bubble-state classification;
- candidate-code decoding;
- candidate mapping uniqueness;
- answer-key parsing and validation;
- scoring rules and edge cases;
- JSON/CSV serialization.

### Integration tests
- image decode and preprocessing;
- registration and alignment;
- ROI extraction;
- complete recognition on known fixtures;
- output artifact generation.

### End-to-end tests
Run the CLI against a small fixture manifest and verify candidate IDs, responses, scores, exit codes and output schema.

### Regression tests
Store representative images and expected labels. When algorithms change, compare outputs and require an explicit review of intentional changes.

## 2. Test data

Use:
- a small set of real dataset scans;
- synthetic fixtures with deterministic ground truth;
- edge cases for blank, multiple, faint, heavy and erased marks;
- invalid files and malformed configuration.

Keep test fixtures small; large benchmark datasets should not run in every unit-test invocation.

## 3. Test principles

- Tests must not depend on network access.
- Avoid brittle exact pixel comparisons when a semantic metric is more appropriate.
- Use tolerances for geometric calculations.
- Test failure paths as deliberately as success paths.
- Ensure uncertain inputs produce explicit ambiguous states.
- Test scoring independently from image recognition.


---

<!-- Source: README.md -->

# OMNISCAN

A local-first, configurable Optical Mark Recognition (OMR) platform. The first deliverable is a deterministic Rust CLI academic MVP; the roadmap later adds shared-core web and desktop applications.

## Status

**Phase 0 — research and technical foundation.** Dataset/template preflight and Rust/OpenCV environment validation are the immediate tasks.

## Read first

1. [`AGENTS.md`](AGENTS.md) — mandatory instructions for coding agents.
2. [`docs/00-project-charter.md`](docs/00-project-charter.md) — product decisions and scope.
3. [`docs/01-dataset-decision.md`](docs/01-dataset-decision.md) — dataset comparison and preflight.
4. [`docs/02-implementation-plan.md`](docs/02-implementation-plan.md) — milestone plan and acceptance criteria.
5. [`docs/06-ai-agent-execution-playbook.md`](docs/06-ai-agent-execution-playbook.md) — how to delegate tasks to an AI coding agent.
6. [`docs/adr/README.md`](docs/adr/README.md) — architecture decisions.

## Principles

- Deterministic, explainable recognition for supported templates.
- Explicit ambiguity instead of silent guesses.
- Original scans are preserved.
- Template and processing configuration are versioned.
- Accuracy and performance are measured, never invented.
- No ML, database, GUI or cloud dependency in the academic MVP.

## Dataset policy

Dataset downloads are separate from build and test. Review the license and dataset preflight before downloading or redistributing files. Large datasets do not belong in Git.

## Development

Exact build instructions will be finalized after the Rust/OpenCV environment check. Expected quality commands:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## License

To be selected deliberately before public release.


---

<!-- Source: docs/00-project-charter.md -->

# OMNISCAN Project Charter

## Purpose

OMNISCAN is a local-first Optical Mark Recognition platform that reads supported answer sheets, identifies candidates, evaluates responses against an answer key and emits auditable results.

## Product strategy

Build a small, correct, measurable academic CLI first. Evolve it into a shared Rust engine used by a desktop app and web app. Optional ML and cloud capabilities are later extensions, not MVP dependencies.

## Product modes

1. Known Template: deterministic processing for a configured sheet.
2. Assisted Template: propose regions and request operator confirmation.
3. Automatic Discovery: future capability for inferring unfamiliar layouts.

Do not promise perfect recognition of every possible form. Page-size independence through normalized geometry is not equivalent to understanding arbitrary layouts.

## Frozen decisions

- Initial deliverable: Rust CLI.
- CV library: OpenCV Rust bindings.
- No ML in academic MVP.
- Public real dataset plus synthetic tests.
- JSON answer key for MVP.
- JSON and CSV output; diagnostic annotated images.
- Positive, negative and partial marking represented in the scoring design, with the MVP implementing only the policies that can be completed and tested.
- Future persistence: PostgreSQL.
- Future interfaces: React + TypeScript web UI and Tauri desktop UI sharing the Rust core.
- Local-first first; cloud deployment later.
- Single user first; multi-user architecture later.
- Modular monolith first; no microservices without evidence.

## Current open decisions

- Final dataset preflight and subset selection.
- Initial sheet template and coordinate calibration.
- Exact supported raster formats for the first vertical slice.
- Exact OpenCV installation/build strategy for the target Windows environment.
- Whether the academic scoring MVP implements partial credit immediately or stages it after basic scoring.

## Definition of academic success

A user can run a documented CLI command on a supported sheet, obtain a candidate identifier where supported, per-question response states, a score, JSON/CSV output and a diagnostic image. Performance and accuracy are measured against disclosed ground truth; no fabricated claims.


---

<!-- Source: docs/01-dataset-decision.md -->

# Dataset Decision and Preflight

**Status:** Recommended, pending local preflight.

## Candidates compared

### A. Tamaulipas MCQ Exam Image Dataset

- Source: https://data.mendeley.com/datasets/djmynjwjpy/2
- Version listed: 2, published 21 May 2026.
- Reported contents: 5,721 scanned four-option answer sheets.
- Ground truth: a text file with human-observed labels accompanies each image.
- Sheet sizes: 3,669 sheets with 90 items and 2,052 with 100 items.
- License listed on the dataset page: CC BY 4.0.
- Reported challenge: varied marking styles and noise/digitization artifacts.
- Fit: best primary real-data evaluation candidate because it provides scale and item-level labels.
- Risks: two question-count cohorts may not mean two geometrically different layouts, but this must be inspected. Candidate identity-to-roll mapping should not be assumed available. Download size and exact file schema must be confirmed locally.

### B. 145-sheet GitHub dataset

- Source: https://github.com/Fardin-Mahmud/Automated-OMR-Sheet-Evaluation-and-Grading-System
- Reported contents: 145 JPG images, 30 questions, four options, three-column layout and ground-truth CSV.
- Fit: quick smoke-test dataset and easy first template candidate.
- Risks: licensing is not clear from the repository landing page reviewed; do not redistribute until the license is verified. Smaller and less varied.

### C. Wild-OMR

- Source: https://zenodo.org/records/21710005
- Reported contents: 769 released captures of 30 physical sheets, 40 questions and five options; flatbed and phone captures.
- Includes controlled capture variation, ground truth, synthetic generator, baselines and evaluation harness.
- License listed: CC BY 4.0 for data; MIT for code.
- Archive size listed: 1.9 GB.
- Fit: later robustness benchmark.
- Risks: too large and more complex than needed for the first end-to-end implementation.

## Recommended use

1. **Primary real-data evaluation:** Tamaulipas, starting with a small representative subset after confirming file schema and image geometry.
2. **Synthetic deterministic tests:** generate known answer patterns and candidate codes for blank, single, multiple, faint, erased and distorted marks.
3. **Optional quick smoke-test:** the 145-sheet dataset only if its license and layout are verified.
4. **Later robustness suite:** Wild-OMR, not a prerequisite for the MVP.

## Preflight checklist

Before writing a template-specific recognition algorithm:
- [ ] Download only after checking access and storage needs.
- [ ] Record access date, version, source URL and license.
- [ ] Inspect archive contents without modifying the original.
- [ ] Count images and annotation files.
- [ ] Parse a small sample of annotations.
- [ ] Verify answer ordering and number of questions.
- [ ] Visually inspect at least 20 sheets across available cohorts.
- [ ] Measure image dimensions and color modes.
- [ ] Determine whether the template is consistent within each cohort.
- [ ] Identify whether registration markers exist.
- [ ] Determine whether candidate identifiers are present and labelled.
- [ ] Record missing/corrupt files and ambiguous labels.
- [ ] Create a manifest; do not rely on filesystem order.
- [ ] Keep a held-out set for final evaluation.

## Candidate IDs

The Tamaulipas dataset description says sheets were anonymized. Do not assume it provides candidate-to-roll mapping. Implement candidate decoding with synthetic fixtures or a separate controlled template unless inspection proves otherwise.

## Citation links

- Mendeley dataset: https://data.mendeley.com/datasets/djmynjwjpy/2
- Wild-OMR Zenodo record: https://zenodo.org/records/21710005
- 145-sheet repository: https://github.com/Fardin-Mahmud/Automated-OMR-Sheet-Evaluation-and-Grading-System


---

<!-- Source: docs/02-implementation-plan.md -->

# Implementation Plan and Milestones

Each milestone should be a separately reviewable pull request or task. Do not ask an agent to implement all milestones in one giant change.

## M0 — Repository and environment foundation

Deliver:
- Cargo project initialized.
- Rust toolchain documented.
- OpenCV Rust binding builds on the target machine.
- Formatting, lint and test commands work in CI.
- Basic error type and CLI `--help` work.

Acceptance:
- `cargo test` passes on a clean checkout.
- CI runs formatting, clippy and tests.
- No dataset download is required to build or test.

## M1 — Dataset inventory and template

Deliver:
- Dataset manifest and label parser.
- Dataset/license note.
- Template geometry document and initial template config.
- A few checked-in synthetic fixtures only; no large public dataset committed.

Acceptance:
- Label parser has tests and clear schema errors.
- At least 20 sample sheets inspected.
- Template supports a documented cohort and question ordering.
- Known limitations are written down.

## M2 — Image ingestion and preprocessing

Deliver:
- Image decode and validation.
- Project-owned image abstraction.
- Grayscale and selected denoising/thresholding implementations.
- Diagnostic output for intermediate images.

Acceptance:
- Invalid/corrupt files produce typed errors.
- Original file is not overwritten.
- Unit/integration tests cover supported formats.
- Otsu and adaptive thresholding can be compared through config or an experiment command.

## M3 — Alignment and template ROI

Deliver:
- Registration detection if the chosen sheet supports it.
- Alignment transform with quality/failure signal.
- Template coordinate transform.
- ROI extraction for response bubbles.

Acceptance:
- Known fixture geometry is within a documented tolerance.
- Bad alignment is flagged rather than silently accepted.
- ROI order matches question/option order.

## M4 — Bubble recognition

Deliver:
- Fill/darkness measurement.
- Configurable thresholding and decision margins.
- States: selected, blank, multiple, ambiguous.
- Diagnostic overlays.

Acceptance:
- Synthetic cases for each response state pass.
- Thresholds are not hard-coded throughout source.
- Evaluation uses held-out samples.
- Ambiguous samples remain ambiguous.

## M5 — Candidate identification and mapping

Deliver:
- Identifier decoding for the MVP's selected identifier layout.
- Mapping file parser and uniqueness validation.
- Explicit unknown/duplicate identifier outcomes.

Acceptance:
- Correct, blank, multiple and invalid identifier cases are tested.
- Candidate lookup is deterministic.
- If the public template lacks an ID region, synthetic fixtures cover this feature and the limitation is explicit.

## M6 — Answer key and scoring

Deliver:
- Versioned JSON answer-key schema.
- Key validation against template question count and options.
- Scoring policy and question-level breakdown.

Acceptance:
- Correct, incorrect, blank, multiple and ambiguous policies are explicit.
- Single-choice and any implemented multi-select behavior have unit tests.
- Partial/negative marking are implemented only if fully tested and documented.
- Scoring code has no dependency on OpenCV.

## M7 — End-to-end CLI and outputs

Deliver:
- `validate-image`, `inspect-template`, `process`, and (if time permits) `batch`.
- JSON/CSV output.
- Annotated diagnostics and clear error messages.

Acceptance:
- A clean-checkout documented command processes a fixture end-to-end.
- JSON output conforms to the documented schema.
- Progress/logging does not corrupt machine-readable stdout.
- Output files are not partially overwritten on failure where atomic writes are practical.

## M8 — Evaluation, optimization and report

Deliver:
- Reproducible evaluation script.
- Recognition metrics and confusion matrix.
- Latency, throughput and peak memory measurements.
- Failure-case gallery and academic report.

Acceptance:
- Results include sample count, split, configuration and environment.
- No performance or accuracy claims are invented.
- Parallel processing is added only if measurements justify it and memory remains bounded.

## Suggested 3-week schedule

Week 1: M0–M3, but reduce scope if dataset/template setup takes longer.  
Week 2: M4–M7.  
Week 3: M8, fixes, reproducibility and report.

The first milestone should be a thin vertical slice, not a perfect framework.


---

<!-- Source: docs/03-domain-contracts.md -->

# Domain Contracts and Data Shapes

These are initial contracts for discussion and implementation. Treat them as proposed stable boundaries, not permission to invent unsupported fields. Update the document alongside implementation.

## Response state

Conceptually, each question response must distinguish:
- `selected`: one recognized option;
- `blank`: no mark exceeds the fill criterion;
- `multiple`: more than one option exceeds the mark criterion;
- `ambiguous`: evidence is insufficient or conflicting.

Do not encode all these states as a single option index or a magic integer.

## Question response

Proposed conceptual fields:
- question identifier/index;
- response state;
- selected option(s), when applicable;
- per-option fill measurements;
- optional confidence/evidence;
- review-required flag and reason.

## Candidate identity

Proposed states:
- `resolved`: identifier maps uniquely;
- `unresolved`: no identifier could be decoded;
- `unknown_code`: decoded code absent from mapping;
- `ambiguous`: multiple possible identifier values;
- `duplicate_mapping`: mapping configuration is invalid.

## Processing result

A result should record:
- source reference/path;
- template identifier and version;
- processing configuration identifier/checksum;
- candidate identity result;
- ordered question responses;
- evaluation result if a key was supplied;
- warnings/errors;
- paths or references to diagnostic artifacts;
- software version when available.

## JSON contract rules

- Use explicit strings for enum-like states.
- Keep question ordering explicit and stable.
- Reject unknown or missing required configuration fields where safety/correctness demands it.
- Include a schema version for persisted output.
- Never include absolute local paths in shared examples unless clearly marked as machine-specific.
- Avoid personally identifying candidate data in test fixtures.
- Document whether scores are integers, decimal values or rational calculations.

## Initial answer key example

```json
{
  "schema_version": 1,
  "key_id": "demo-key-v1",
  "questions": [
    {
      "question_id": "Q1",
      "correct_options": ["B"],
      "marks_correct": 4.0,
      "marks_incorrect": -1.0,
      "marks_blank": 0.0
    },
    {
      "question_id": "Q2",
      "correct_options": ["A", "C"],
      "marks_correct": 4.0,
      "marks_incorrect": -1.0,
      "marks_blank": 0.0
    }
  ]
}
```

The example illustrates a possible schema, not a requirement to implement multi-select in the first vertical slice. The implementation must decide and document how partial credit is calculated for multi-select responses before enabling it.


---

<!-- Source: docs/04-testing-and-benchmark-contract.md -->

# Testing and Benchmark Contract

## Test layers

1. Unit tests: geometry, parsing, fill measurement, classification, candidate mapping and scoring.
2. Integration tests: image loading, preprocessing, alignment and ROI extraction.
3. End-to-end tests: CLI input to result JSON/CSV and diagnostics.
4. Regression tests: fixed fixtures with expected semantic labels.
5. Benchmarks: run separately from normal unit tests.

## Required edge cases

- valid sheet;
- unreadable or corrupt image;
- wrong image dimensions;
- rotated sheet;
- mild perspective distortion where supported;
- blank response;
- single marked response;
- multiple marks;
- faint mark;
- erased mark;
- stray marks/noise;
- unknown candidate code;
- duplicate candidate mapping;
- malformed template;
- answer-key question count mismatch;
- output directory unavailable.

## Metric definitions

- Bubble precision/recall/F1: only if per-bubble ground truth exists.
- Question exact accuracy: a question is correct only if its complete response state and selected options match ground truth.
- Sheet exact accuracy: all scored questions on a sheet match ground truth.
- Candidate ID exact match: decoded ID exactly equals the known identifier.
- Ambiguous rate: fraction of questions deliberately left unresolved.
- Throughput: completed sheets divided by elapsed wall-clock seconds.
- Latency: report median and p95 where sample count permits.
- Peak memory: record the measurement method and environment.

## Experiment discipline

- Separate tuning data from held-out evaluation data.
- Keep a manifest and stable seed for synthetic data.
- Do not report synthetic accuracy as real-world accuracy.
- Report exact sample count and excluded cases.
- Record OS, CPU, RAM, Rust version, OpenCV version, build profile, worker count and configuration checksum.
- Run performance tests in release mode and note warm-up/repetition policy.


---

<!-- Source: docs/05-github-workflow.md -->

# GitHub Repository Setup and Workflow

## 1. Create repository

Suggested repository name: `omniscan`.

Suggested description:
> A local-first, configurable Optical Mark Recognition platform with a high-performance Rust processing core, CLI, and planned web and desktop applications.

Create the repository without committing a dataset. A public repository is appropriate for portfolio visibility if you are comfortable sharing the source; choose private if you want to delay public release.

## 2. First local setup

```bash
git clone https://github.com/<YOUR_USERNAME>/omniscan.git
cd omniscan
git switch -c docs/initial-project-foundation
```

If you created the repository without a README, create the first files locally instead. Avoid unrelated histories by deciding on one initialization path.

## 3. Branch conventions

- `main`: stable, tested state.
- `docs/<topic>`: documentation-only changes.
- `feat/<topic>`: feature implementation.
- `fix/<topic>`: bug fixes.
- `test/<topic>`: test-only work where useful.

Keep branches short-lived and focused.

## 4. Commit examples

```text
docs: establish OMNISCAN product and engineering baseline
chore: initialize Rust project and CI checks
feat(imaging): add validated image loading
feat(layout): add normalized template geometry
feat(recognition): classify bubble response states
feat(evaluation): implement configurable scoring
test(pipeline): add end-to-end OMR fixtures
perf(batch): add bounded parallel processing
```

## 5. Initial GitHub settings

- Protect `main` against force pushes and accidental deletion.
- Require CI checks before merging when practical.
- Enable Issues and Discussions only if you plan to use them.
- Add issue templates for bug reports and feature requests once the code exists.
- Use GitHub Actions for format, lint and test checks.
- Add a license only after choosing one deliberately.
- Do not enable Git LFS or commit large data by default. Store dataset download instructions and checksums in documentation instead.

## 6. Dataset policy

- Keep small, synthetic, non-sensitive test fixtures in the repository.
- Store large datasets outside Git and download them through a documented manual or opt-in script.
- Record dataset version, license, source URL and checksum.
- Never commit student-identifying data.
- Do not redistribute a dataset unless its license permits it.

## 7. First milestones as Issues

Create one issue per milestone from `docs/02-implementation-plan.md`. Each issue must state scope, dependencies, acceptance criteria and explicit out-of-scope items. Ask the agent to work on one issue at a time.


---

<!-- Source: docs/06-ai-agent-execution-playbook.md -->

# AI Agent Execution Playbook

## Goal

Use an AI coding agent as an implementation partner without allowing it to silently expand scope or invent architecture.

## Operating loop

1. Give the agent `AGENTS.md` and the relevant document(s).
2. Ask it to inspect the repository and report current state before editing.
3. Assign exactly one milestone or small task.
4. Require a proposed plan and acceptance checklist.
5. Approve or correct the plan.
6. Let it implement and test.
7. Review the diff, tests and documentation.
8. Merge only when acceptance criteria pass.

## First prompt: repository reconnaissance

```text
Read AGENTS.md and docs/00-project-charter.md through
docs/06-ai-agent-execution-playbook.md, plus the ADRs.

Do not implement features yet. Inspect the repository and report:
1. current repository state;
2. missing files and environment prerequisites;
3. proposed M0 plan;
4. OpenCV/Rust risks on this machine;
5. commands you would run to validate M0.

Do not change files, install packages, download datasets, or make commits
until I approve the plan.
```

## M0 implementation prompt

```text
Implement only M0 from docs/02-implementation-plan.md.

First inspect the repository and propose the smallest implementation plan.
Follow AGENTS.md and existing ADRs. Initialize the minimal Rust CLI, add
typed top-level errors and a `--help` command, and configure CI for fmt,
clippy and tests. Do not implement OMR algorithms, do not download datasets,
do not add a database, GUI, ML or cloud services.

Acceptance:
- clean-checkout build and tests work;
- formatting/lint/test commands are documented;
- CI configuration is present;
- no unrelated refactors;
- report exact commands and their results.
Wait for approval before broadening scope.
```

## Feature task prompt template

```text
Task: <single task title>

Read AGENTS.md and these documents:
- <relevant docs>
- <relevant ADRs>

Before editing:
1. inspect the relevant code;
2. state the intended design and files;
3. list acceptance criteria and tests;
4. identify assumptions or questions.

Implement only this task. Do not expand scope or change accepted ADRs.
Add tests, update docs and run the applicable quality checks.
Report files changed, commands/results, limitations and next task.
Do not commit or push unless explicitly instructed.
```

## Review prompt

```text
Review this diff against AGENTS.md, the task acceptance criteria and relevant ADRs.
Do not edit code yet. Identify correctness bugs, algorithmic weaknesses,
missing tests, API/schema inconsistencies, unnecessary dependencies,
memory risks, licensing concerns and unsupported claims. Rank findings
by severity and point to exact files/lines. Distinguish blockers from suggestions.
```

## Rules for agent selection

Choose an agent that can inspect/edit repository files and run terminal commands. Give it the entire repository context or point it to `AGENTS.md`; do not rely on a one-time chat prompt that is lost between tasks. If the platform supports repository-level instruction files, keep `AGENTS.md` at the root.

## Human review checklist

- Does the diff implement only the assigned task?
- Are the algorithms explained and testable?
- Are uncertain inputs handled explicitly?
- Are dependencies justified?
- Are test outputs real and reproducible?
- Are documentation and actual behavior consistent?
- Has any data been downloaded or uploaded without permission?
- Are there unapproved schema/ADR changes?


---

<!-- Source: docs/07-initial-repository-tree.md -->

# Initial Repository Tree

Use this as a target structure. Create directories when they contain real code; avoid empty scaffolding for every future module.

```text
omniscan/
├── AGENTS.md
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE                  # only after license decision
├── .gitignore
├── rust-toolchain.toml      # if a pinned toolchain is chosen
├── configs/
│   ├── templates/
│   ├── preprocessing.json
│   └── recognition.json
├── docs/
│   ├── 00-project-charter.md
│   ├── 01-dataset-decision.md
│   ├── 02-implementation-plan.md
│   ├── 03-domain-contracts.md
│   ├── 04-testing-and-benchmark-contract.md
│   ├── 05-github-workflow.md
│   ├── 06-ai-agent-execution-playbook.md
│   ├── 07-initial-repository-tree.md
│   └── adr/
├── examples/
│   ├── answer-keys/
│   └── mappings/
├── scripts/
├── src/
│   ├── main.rs
│   ├── cli/
│   ├── core/
│   ├── imaging/
│   ├── layout/
│   ├── recognition/
│   ├── evaluation/
│   ├── pipeline/
│   ├── output/
│   └── benchmark/
├── tests/
│   ├── fixtures/
│   └── end_to_end.rs
└── .github/
    └── workflows/
        └── ci.yml
```

## Dataset location

Do not put a large dataset in the repository. A local directory such as `data/external/` may be gitignored. Commit only small synthetic fixtures whose provenance and generated ground truth are documented.


---

<!-- Source: docs/adr/ADR-001-rust-core.md -->

# ADR-001: Rust as the Primary Processing Core

- **Status:** Accepted

## Context
The academic assignment needs an efficient, maintainable CLI and the long-term platform should reuse its processing engine across interfaces.

## Decision
Implement the core OMR pipeline in Rust.

## Consequences
- Strong types and explicit error handling support maintainability.
- Rust supports efficient CPU-bound processing and controlled concurrency.
- The team must invest in Rust and OpenCV binding setup.
- Python may remain useful for dataset exploration and optional ML experimentation, but it is not the authoritative MVP engine.


---

<!-- Source: docs/adr/ADR-002-opencv.md -->

# ADR-002: OpenCV Through Rust Bindings

- **Status:** Accepted

## Context
OMR processing requires image decoding, filtering, thresholding, contours and geometric transformations.

## Decision
Use OpenCV through Rust bindings for the academic CV pipeline.

## Consequences
- Leverage established CV primitives instead of reimplementing foundational image operations.
- Keep OpenCV types behind adapters and project-owned image abstractions.
- Pin and document the native OpenCV/toolchain setup for reproducibility.
- Benchmark alternative algorithms rather than assuming defaults are optimal.


---

<!-- Source: docs/adr/ADR-003-no-ml-mvp.md -->

# ADR-003: No Machine Learning in the Academic MVP

- **Status:** Accepted

## Context
The assignment deadline is approximately 1–2 weeks and the objective is to demonstrate algorithmic image processing with explainable behavior.

## Decision
Do not use ML for the academic MVP. Use deterministic image processing and template-based recognition.

## Consequences
- Easier to explain, debug and benchmark.
- Scope is bounded to documented templates and supported scan conditions.
- Difficult or ambiguous cases must be flagged rather than guessed.
- Optional ML can be evaluated in a later subsystem without replacing the deterministic baseline.


---

<!-- Source: docs/adr/ADR-004-modular-monolith.md -->

# ADR-004: Start With a Modular Monolith

- **Status:** Accepted

## Context
The product is early-stage and the academic deliverable is a single CLI. Distributed services would add operational complexity before there is evidence they are needed.

## Decision
Use a modular monolith with clear module boundaries.

## Consequences
- Easier local development, testing and deployment.
- Modules can later be extracted if scale or independent deployment justifies it.
- Enforce dependency direction and avoid cross-module implementation leakage.


---

<!-- Source: docs/adr/ADR-005-shared-core.md -->

# ADR-005: Share One Processing Core Across Interfaces

- **Status:** Accepted

## Context
The roadmap includes CLI, desktop and web experiences. Duplicating OMR logic would create inconsistent results and multiply maintenance.

## Decision
Keep recognition, candidate resolution and evaluation in a shared Rust core. Interfaces call into that core through suitable application boundaries.

## Consequences
- Consistent behavior across CLI and GUI.
- Core APIs must not depend on a specific UI.
- Web deployment may require a service/API boundary; desktop may call a local library or local service.


---

<!-- Source: docs/adr/ADR-006-postgresql.md -->

# ADR-006: PostgreSQL for Future Platform Persistence

- **Status:** Accepted direction; deployment lifecycle open

## Context
The long-term product needs relational integrity for projects, templates, answer keys, jobs, candidates, results and audit history. PostgreSQL is the chosen database.

## Decision
Use PostgreSQL as the canonical structured application database for the platform. The academic CLI remains file-based and does not require a database.

## Consequences
- Use migrations and explicit schema versioning.
- Store metadata and structured records in PostgreSQL.
- Store large image and report artifacts in filesystem/object storage and keep references in the database.
- Local-first deployment needs a deliberate PostgreSQL installation, startup, backup and upgrade strategy. This must be validated before implementation.
- Do not add SQLite solely as a shortcut without revisiting the storage decision explicitly.


---

<!-- Source: docs/adr/ADR-007-hybrid-layout.md -->

# ADR-007: Hybrid OMR Layout Strategy

- **Status:** Accepted

## Context
Different institutions use different sheet layouts. Fully hard-coded coordinates do not scale, while universal automatic layout inference is too risky for the initial product.

## Decision
Support known-template processing first, assisted template configuration next, and automatic layout discovery as a later capability.

## Consequences
- Initial implementation is deterministic and bounded.
- Template geometry is explicit and versioned.
- Unfamiliar layouts can be assisted rather than silently misread.
- “Any page size” is handled through geometric normalization, not assumed to mean arbitrary layout understanding.


---

<!-- Source: docs/adr/ADR-008-confidence-review.md -->

# ADR-008: Confidence-Aware Recognition

- **Status:** Accepted

## Context
A wrong candidate ID or response can cause an incorrect result. A forced prediction hides uncertainty.

## Decision
Recognition must support explicit ambiguous and review-required states. Confidence/evidence should be retained where meaningful.

## Consequences
- Thresholds and ambiguity margins need calibration.
- Reports can distinguish automatically accepted from unresolved results.
- Human review becomes a first-class future workflow.
- The system should prefer abstention over an unsupported guess.


---

<!-- Source: docs/adr/ADR-009-preserve-originals.md -->

# ADR-009: Preserve Original Source Documents

- **Status:** Accepted

## Context
Preprocessing and alignment may modify image appearance. The original scan is essential for audit, debugging and reprocessing.

## Decision
Never overwrite original scans. Derived images and annotations are separate artifacts.

## Consequences
- Additional storage is required.
- Every derived artifact should be traceable to its source and processing configuration.
- Reprocessing remains possible when algorithms or templates improve.


---

<!-- Source: docs/adr/ADR-010-versioned-config.md -->

# ADR-010: Version Templates and Processing Configurations

- **Status:** Accepted

## Context
A result is only reproducible if the template and processing parameters used to generate it are known.

## Decision
Version templates, answer keys and processing configurations. Record the applicable version or checksum with each processing run.

## Consequences
- Configuration changes do not silently alter historical results.
- Benchmark results can be tied to exact configurations.
- The platform needs migration and compatibility policies for old configurations.


---

<!-- Source: docs/adr/README.md -->

# Architecture Decision Records

ADRs record important technical choices and their status. A decision may be revisited when evidence changes.

| ID | Decision | Status |
|---|---|---|
| ADR-001 | Rust as primary processing core | Accepted |
| ADR-002 | OpenCV through Rust bindings | Accepted |
| ADR-003 | No ML in academic MVP | Accepted |
| ADR-004 | Modular monolith initially | Accepted |
| ADR-005 | Shared processing core for CLI/web/desktop | Accepted |
| ADR-006 | PostgreSQL for future platform persistence | Accepted direction; lifecycle details open |
| ADR-007 | Hybrid template strategy: known + assisted + future discovery | Accepted |
| ADR-008 | Confidence-aware recognition and human review | Accepted |
| ADR-009 | Preserve original source documents | Accepted |
| ADR-010 | Version templates and processing configuration | Accepted |

See each ADR for context, decision, consequences and open questions.


---

<!-- Source: docs/algorithm-design.md -->

# Algorithm Design

## 1. Pipeline overview

```text
Input scan
   |
Decode + validate
   |
Quality checks
   |
Grayscale / denoise / threshold
   |
Registration detection + geometric alignment
   |
Template coordinate transform
   |
ROI extraction
   |
Bubble fill measurement
   |
Response-state classification
   |
Candidate identifier decoding + mapping
   |
Answer-key evaluation
   |
JSON / CSV + annotated diagnostics
```

## 2. Image preprocessing

Investigate grayscale conversion, denoising and thresholding. Compare Otsu global thresholding with adaptive thresholding on the selected dataset. Do not assume one method is universally superior; record results by scan condition.

Keep the original image unchanged. Every derived image should be a separate processing artifact.

## 3. Geometric alignment

If the template has registration markers or stable page features, detect them and estimate a geometric transform. Use an affine transform for rotation/scale/translation when adequate; use a homography when perspective distortion requires a projective mapping and enough reliable correspondences are available.

Reject or flag poor alignment rather than extracting ROIs from a badly registered page.

## 4. Template and normalized coordinates

Represent regions relative to a reference page coordinate system. A template should define:
- reference width and height;
- page orientation;
- registration features;
- candidate identifier region;
- question and option regions;
- question type and option count;
- any calibration or tolerance parameters.

Normalized coordinates help handle different raster resolutions. They do not automatically solve unknown layout recognition.

## 5. Bubble fill measurement

For each expected bubble ROI, compute a darkness or foreground-density measure. One simple metric is:

\[
d = \frac{\text{foreground pixels inside bubble mask}}
         {\text{total pixels inside bubble mask}}
\]

A circular mask is preferable when bubble geometry is known. A rectangular ROI may be used for early experiments, but it can include border and neighboring artifacts.

An integral image can accelerate rectangular sum queries: build it in \(O(W H)\), then query each axis-aligned rectangle in \(O(1)\). For circular regions, use precomputed masks or another measured approach.

## 6. Response classification

Use calibrated thresholds and explicit margins between the strongest candidate and other candidates. A question can be:
- `Selected(option)`;
- `Blank`;
- `Multiple(options)`;
- `Ambiguous(reason)`.

Do not force a low-confidence bubble into a selected state. Thresholds must be calibrated on training/validation samples and evaluated on held-out scans.

## 7. Candidate identification

Decode the identifier using the method supported by the chosen template. For bubble-coded digits, evaluate each digit column/row using the same fill analysis, with explicit handling for blank or multiple marks. Resolve the decoded code through a mapping table.

Use a hash map for candidate-code lookup: expected \(O(1)\) lookup. Detect duplicate codes before processing.

## 8. Evaluation

For each question, compare the recognized response with the answer key and apply the configured marking policy. Define behavior for:
- correct response;
- incorrect response;
- blank response;
- multiple marks;
- ambiguous response;
- multiple-select response;
- partial credit.

The evaluation module should be independent of image processing and tested using synthetic domain objects.

## 9. Data structures and complexity

| Need | Candidate structure | Rationale |
|---|---|---|
| Candidate code lookup | `HashMap` | Expected constant-time lookup |
| Ordered question responses | `Vec` | Contiguous storage and indexed access |
| Duplicate detection | `HashSet` | Expected constant-time membership |
| Irregular spatial lookup | Spatial grid or R-tree | Add only if geometry/query volume justifies it |
| Batch scheduling | Bounded queue | Controls peak memory |
| Per-question scoring | Sequential/vector iteration | \(O(Q)\) per sheet |

Avoid introducing complex data structures merely to make the project appear sophisticated. Measure actual bottlenecks first.

## 10. Duplicate scans

Use cryptographic hashes to identify byte-identical files. Perceptual hashes can flag visually similar scans, but must not automatically discard them: different candidates can have near-identical blank forms. Treat similarity as a review signal.


---

<!-- Source: docs/architecture.md -->

# System Architecture

## 1. Architectural direction

Use a **modular monolith** initially. The Rust processing core is shared by the CLI and future application interfaces. Avoid microservices until real deployment or scaling requirements justify them.

## 2. Target architecture

```text
                    User Interfaces
             +----------+----------+
             |          |          |
            CLI       Desktop      Web
                        |           |
                     Tauri       React/TS
                        \           /
                         API / App Layer
                               |
                      Shared Rust Core
       +-----------------------+-----------------------+
       |                       |                       |
   Ingestion               OMR Engine              Evaluation
       |                       |                       |
   Validation       Preprocess -> Align -> ROI     Answer Key
   File handling    Layout -> Bubble recognition  Scoring
                    Candidate identification       Results
       +-----------------------+-----------------------+
                               |
                Persistence / Artifact Storage
                 PostgreSQL + File/Object Storage
                               |
                 Optional Intelligence Subsystem
                  Python ML experiments/services
```

The academic MVP implements the Rust core and CLI path only.

## 3. Core modules

- `core`: shared domain types, errors, configuration and traits.
- `imaging`: loading, preprocessing, thresholding and geometry.
- `layout`: templates, regions and registration features.
- `recognition`: bubble analysis, identifier extraction and confidence.
- `evaluation`: answer keys, candidate mapping and scoring.
- `pipeline`: orchestration of the processing stages.
- `output`: JSON/CSV serialization and annotated images.
- `benchmark`: metrics and experiment reporting.
- `cli`: argument parsing and command handlers.

## 4. Dependency direction

The domain/core layer should not depend on OpenCV. OpenCV-specific types should be confined to imaging adapters. Recognition and evaluation should consume project-owned types rather than exposing `opencv::core::Mat` throughout the codebase.

Example abstraction:

```rust
pub trait ImagePreprocessor {
    fn preprocess(
        &self,
        input: &ImageFrame,
    ) -> Result<ProcessedImage, ProcessingError>;
}

pub trait BubbleRecognizer {
    fn recognize(
        &self,
        image: &ProcessedImage,
        layout: &OmrLayout,
    ) -> Result<Vec<QuestionResponse>, RecognitionError>;
}
```

These interfaces are illustrative; refine them after the selected dataset and template are known.

## 5. Storage boundaries in the future platform

PostgreSQL is the canonical store for structured application data: projects, templates, answer-key versions, jobs, candidate mappings, results, review actions and audit events.

Store large binary artifacts—original scans, normalized images, annotations and generated reports—in filesystem or object storage. Store stable references and metadata in PostgreSQL rather than placing all image bytes in relational rows.

The local-first PostgreSQL lifecycle remains an ADR to validate. The academic CLI should not depend on a database.

## 6. Concurrency and memory

Use bounded work queues for batch processing. Parallelize independent page-level work only after measuring memory per in-flight image. A configurable worker count should be available. Do not read an unbounded batch into RAM.

## 7. Future ML boundary

Keep ML optional and isolated behind a stable interface. Python may be used for training and experimentation; the deterministic Rust pipeline remains the default. Any ML-assisted output must expose confidence and preserve human review for uncertain cases.


---

<!-- Source: docs/benchmark-methodology.md -->

# Benchmark and Evaluation Methodology

## 1. Purpose

Measure recognition quality and computational performance using a reproducible protocol. Report actual results only after running the implementation.

## 2. Dataset partitioning

Separate data used for threshold calibration from final evaluation. Where possible, split by physical sheet or source session rather than randomly splitting near-duplicate crops. Keep a manifest of all samples and ground-truth labels.

## 3. Recognition metrics

Report at least:
- per-bubble classification precision, recall and F1;
- per-question exact response accuracy;
- per-sheet exact response accuracy;
- blank detection accuracy;
- multiple-mark detection accuracy;
- candidate-ID exact match rate;
- unresolved/ambiguous rate.

For multi-class or imbalanced labels, include a confusion matrix and macro-F1. Explain the unit of evaluation (bubble, question or sheet).

## 4. Operational metrics

Measure:
- end-to-end latency per sheet;
- stage-level latency;
- throughput in sheets/second;
- peak resident memory;
- batch completion time;
- failure rate.

Report warm-up policy, number of repetitions, dataset size, worker count, build profile and hardware/software environment.

## 5. Experimental comparisons

Useful experiments include:
- Otsu vs adaptive thresholding;
- denoising methods;
- alignment on vs off (where relevant);
- different fill thresholds;
- ROI mask variants;
- sequential vs bounded parallel batch execution.

Change one factor at a time where possible. Avoid selecting parameters on the final test set.

## 6. Reporting format

Every result table should state:
- dataset and split;
- sample count;
- configuration identifier;
- metric definition;
- measured value;
- test environment.

Include representative successes and failure cases with annotated images. Explain failure causes rather than reporting only an aggregate score.

## 7. Reproducibility

Record Git commit, Rust version, dependency versions, configuration checksum, dataset manifest checksum and benchmark command. Do not invent or estimate measurements.


---

<!-- Source: docs/cli-specification.md -->

# CLI Specification (Proposed)

The exact command names may change during implementation. Keep the CLI discoverable, composable and script-friendly.

## 1. Proposed commands

```text
omniscan
  validate-image <IMAGE>
  inspect-template <TEMPLATE.json>
  process <IMAGE> --template <TEMPLATE.json> --answer-key <KEY.json>
  batch <INPUT_DIR> --template <TEMPLATE.json> --answer-key <KEY.json>
  evaluate <RESPONSES.json> --answer-key <KEY.json>
  benchmark <MANIFEST.json> --template <TEMPLATE.json>
```

## 2. Output conventions

- Machine-readable JSON for structured results.
- CSV for tabular batch results.
- Diagnostic images written to an explicit output directory.
- Progress and human-readable diagnostics to stderr.
- Avoid mixing progress text into JSON stdout.
- Return non-zero exit codes for invalid input, configuration errors and processing failures.

## 3. Configuration

Keep template, preprocessing and recognition configuration separate where useful:

```text
configs/
  templates/
    initial-template.json
  preprocessing.json
  recognition.json
```

Every run should record the configuration version or a checksum so results can be reproduced.

## 4. Error handling

Errors should identify the stage and affected file. Distinguish:
- invalid or unsupported file;
- image decode failure;
- invalid template;
- alignment failure;
- recognition ambiguity;
- candidate mapping failure;
- answer-key validation failure;
- output write failure.

Ambiguity is not necessarily a fatal process error: it may produce a result requiring review.


---

<!-- Source: docs/data-model.md -->

# Conceptual Data Model

This document describes the future platform's conceptual entities. The academic CLI may represent these as in-memory Rust structs and JSON configuration rather than database tables.

## 1. Entities

### Project
An exam/evaluation workspace. Has an identifier, name, description, status and timestamps.

### Template
A versioned description of page geometry, registration features, candidate-ID regions, question regions, option locations and supported response types.

### AnswerKey
A versioned set of expected answers and scoring rules associated with an exam/project.

### Candidate
A person or exam participant. Contains an internal identifier and permitted identifying attributes.

### CandidateCodeMapping
Maps a code read from the sheet to exactly one candidate/roll number. Enforce uniqueness where required.

### Scan
A source document and its metadata: file reference, checksum, dimensions, upload time, detected quality and processing status.

### ProcessingJob
Tracks a batch or single-scan run, configuration version, status, timestamps, progress and errors.

### RecognitionResult
Stores candidate identification, per-question response states, confidence/evidence and processing provenance.

### EvaluationResult
Stores awarded marks, maximum marks, question-level breakdown and scoring-policy version.

### ReviewAction
Records a human correction or confirmation, actor, timestamp, old value, new value and reason.

### Artifact
References original scans, aligned images, overlays, annotated images, exports and reports.

## 2. Relationship sketch

```text
Project 1---* TemplateVersion
Project 1---* AnswerKeyVersion
Project 1---* Scan
Project 1---* ProcessingJob
Project 1---* Candidate
Candidate 1---* CandidateCodeMapping
Scan 1---* RecognitionResult
ProcessingJob 1---* RecognitionResult
RecognitionResult 1---1 EvaluationResult
RecognitionResult 1---* ReviewAction
Scan 1---* Artifact
```

## 3. Persistence principles

- PostgreSQL stores structured metadata and transactional state.
- Binary image/report artifacts live in filesystem or object storage.
- Store checksums and artifact references for integrity and traceability.
- Version templates, answer keys and processing configurations.
- Preserve the exact configuration and software version used for each processing run.
- Do not overwrite original scans during preprocessing.


---

<!-- Source: docs/dataset-research-plan.md -->

# Dataset Research and Selection Plan

**Status: Pending research.** No dataset is selected in this document.

## 1. Goal

Choose a public scanned OMR dataset suitable for a short, measurable Rust CLI assignment. Dataset selection determines the initial template, ground-truth format, recognition approach and evaluation claims.

## 2. Minimum comparison set

Investigate at least three candidate public datasets or reproducible research resources. For each, record:

| Criterion | Questions |
|---|---|
| Access | Is there a direct download or a documented request process? |
| License | Is academic use permitted? Are redistribution restrictions clear? |
| Images | Are actual scanned/photo OMR sheets available? |
| Ground truth | Are marked answers labelled per question? |
| Template | Is the sheet geometry documented or inferable? |
| Candidate ID | Is an identifier present and labelled? |
| Scale | How many sheets and pages are available? |
| Variation | Does it include rotation, skew, lighting, noise and print variation? |
| Format | Image/PDF format, resolution and color depth? |
| Reproducibility | Are scripts, paper, annotation schema or benchmark splits available? |

## 3. Selection principles

Prefer a dataset that:
1. can be accessed immediately;
2. includes real scanned sheets;
3. has trustworthy answer-level ground truth;
4. uses a consistent, documented template;
5. is small enough to inspect and process within the deadline;
6. has clear academic usage terms.

A dataset without candidate identifiers can still support answer recognition. In that case, use synthetic sheets for candidate-ID and roll-mapping tests and disclose the split between real and synthetic evaluation.

## 4. Synthetic dataset strategy

Build a generator only after documenting the selected template. Generate known answer patterns and candidate codes, then render bubbles and introduce controlled variations:
- translation, rotation and mild perspective;
- scale and resolution changes;
- blur and sensor noise;
- brightness/contrast changes;
- faint and heavy marks;
- erasures and stray marks;
- multiple marks and blank responses.

Store ground truth beside each generated image in a machine-readable manifest. Use deterministic random seeds. Keep training/tuning samples separate from held-out evaluation samples.

Synthetic data is useful for controlled tests, but it must not be presented as a substitute for real-world validation.

## 5. Deliverables

- Dataset comparison table with URLs, access date and license notes.
- Chosen dataset and rationale.
- Sample inventory and annotation audit.
- Template geometry document.
- Ground-truth schema.
- Synthetic generation specification.
- Train/validation/test split manifest.

## 6. Important limitation

Do not claim that a dataset supports candidate identification, multiple layouts or arbitrary page sizes unless its actual files and labels demonstrate those properties.


---

<!-- Source: docs/engineering-workflow.md -->

# Engineering Workflow and Quality Standards

## 1. Repository principles

- Keep modules cohesive and dependencies directed inward.
- Use explicit domain types instead of passing loosely structured values.
- Keep OpenCV-specific types behind adapters.
- Keep configuration external and validated.
- Preserve original input files.
- Prefer small, reviewable commits.

## 2. Suggested repository structure

```text
omniscan/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── configs/
├── datasets/
│   ├── public/
│   ├── synthetic/
│   ├── answer_keys/
│   └── mappings/
├── docs/
├── src/
│   ├── main.rs
│   ├── cli/
│   ├── core/
│   ├── imaging/
│   ├── layout/
│   ├── recognition/
│   ├── evaluation/
│   ├── pipeline/
│   ├── output/
│   └── benchmark/
├── tests/
└── scripts/
```

Create files incrementally as implementation requires; the tree is a target organization, not a requirement to create empty modules immediately.

## 3. Rust quality

- Run `cargo fmt --check`.
- Run `cargo clippy --all-targets --all-features -- -D warnings` where practical.
- Run `cargo test`.
- Document public types and non-obvious algorithms.
- Avoid unnecessary cloning and unbounded allocations.
- Use `Result` for recoverable errors; avoid panics on user input.
- Validate all external configuration and file paths.

## 4. Git workflow

Use focused conventional commits, for example:

```text
docs: establish OMNISCAN product and engineering baseline
chore: initialize Rust workspace and CI checks
feat(imaging): add validated image loading
feat(layout): add normalized template geometry
feat(recognition): classify bubble response states
feat(evaluation): implement configurable scoring
test(pipeline): add end-to-end OMR fixtures
perf(batch): add bounded parallel processing
```

## 5. Definition of done

A change is done when:
- behavior and assumptions are documented;
- relevant tests pass;
- invalid inputs and failure modes are handled;
- output is deterministic where expected;
- performance-sensitive changes have measurements;
- no original input is overwritten;
- logs and diagnostics are useful;
- the change is committed with a focused message.

## 6. CI/CD direction

For the academic MVP, CI should build and test the Rust project and run formatting/lint checks. Later platform phases can add frontend checks, integration environments, security scanning, packaging and deployment workflows.


---

<!-- Source: docs/mvp-scope.md -->

# Academic MVP Scope

## 1. Objective

Deliver a reliable, explainable Rust CLI that demonstrates an end-to-end OMR pipeline for at least one documented sheet template.

## 2. Included

- Public scanned OMR dataset investigation and selection.
- Synthetic OMR data generation with known ground truth.
- One documented initial template.
- Image loading and validation.
- Grayscale conversion and denoising experiments.
- Otsu and adaptive thresholding comparison.
- Registration marker detection, where the selected template provides suitable markers.
- Alignment and perspective correction where justified.
- Template-based region-of-interest (ROI) extraction.
- Bubble fill measurement and classification.
- Blank, multiple-mark and ambiguous response handling.
- Candidate identifier extraction for the selected template.
- Candidate-to-roll mapping.
- JSON answer key.
- Configurable positive/negative/partial scoring.
- CSV and JSON output.
- Annotated diagnostic images.
- Unit, integration and end-to-end tests.
- Accuracy, latency, throughput and memory measurements.

## 3. Excluded

- Web UI, desktop UI and Tauri packaging.
- PostgreSQL and application persistence.
- Authentication and multi-user support.
- Cloud services.
- ML models and neural inference.
- Automatic understanding of arbitrary unknown layouts.
- Full-featured PDF ingestion unless the chosen dataset makes it essential.

## 4. MVP workflow

1. Read CLI arguments and configuration.
2. Load and validate the input scan.
3. Preprocess the image.
4. Detect registration features and align the sheet if needed.
5. Load the selected template.
6. Extract identifier and response ROIs.
7. Measure bubble fill.
8. Classify each response, retaining ambiguity.
9. Resolve candidate identifier to a roll number.
10. Load answer key and scoring policy.
11. Evaluate responses.
12. Write machine-readable results and annotated diagnostics.

## 5. Acceptance criteria

- A documented template and its coordinate conventions are checked into the repository.
- The CLI rejects invalid input with actionable errors.
- A valid sample completes the full pipeline.
- Each question produces a defined state: selected, blank, multiple or ambiguous.
- Candidate mapping failures are explicitly represented.
- Scoring edge cases are covered by tests.
- Output is deterministic for the same input, template and configuration.
- Ground-truth evaluation reports per-question and per-sheet metrics.
- Benchmarks record machine, build mode, dataset size and configuration.
- Original source scans remain unchanged.

## 6. Scope control

If schedule pressure arises, reduce the number of supported templates and input formats before weakening correctness, tests or evaluation methodology. A narrow, measured implementation is preferable to a broad but unvalidated demo.


---

<!-- Source: docs/product-requirements.md -->

# Product Requirements Document (PRD)

**Product:** OMNISCAN  
**Version:** 0.1  
**Status:** Draft baseline

## 1. Executive summary

OMNISCAN is an OMR (Optical Mark Recognition) recognition, evaluation and analytics platform. It processes scanned examination sheets, identifies candidates, extracts marked responses, evaluates them against an answer key, and produces structured results and reports.

The long-term product is local-first and supports both web and desktop interfaces over a shared Rust processing core. The academic assignment is deliberately narrower: a Rust CLI that demonstrates a well-engineered, measurable OMR pipeline without machine learning.

## 2. Problem statement

Manual checking of objective examination sheets is repetitive and can be slow and error-prone. An OMR workflow must reliably:

- locate and interpret response bubbles;
- distinguish blank, marked and ambiguous responses;
- identify the candidate;
- apply the correct answer key and marking policy;
- retain enough evidence to audit and correct uncertain cases.

A useful system must handle image quality variation and configurable sheet layouts without pretending that every possible form can be understood automatically.

## 3. Product vision

Build a configurable OMR platform that can process known sheet templates deterministically, assist users in configuring unfamiliar templates, and progressively add automatic layout discovery and optional ML assistance.

### Recognition modes

1. **Known Template Mode:** process a configured layout using deterministic geometry and image analysis.
2. **Assisted Template Mode:** propose detected regions and let an operator confirm or correct them.
3. **Automatic Discovery Mode:** infer candidate layout regions for a previously unseen form; uncertain detections require review.

Universal recognition is not a promise of perfect recognition for arbitrary sheets. Page-size independence is achieved through normalized coordinates and geometric transformations; it does not imply layout independence.

## 4. Target users

- Examination administrators and institutions.
- Teachers and evaluators handling objective tests.
- Small organizations conducting assessments.
- Developers integrating OMR processing into larger systems.

The first release is single-user. The architecture should not block future multi-user operation.

## 5. Product goals

- Correctly interpret supported OMR templates under documented conditions.
- Make recognition uncertainty visible and reviewable.
- Support candidate identification and explicit candidate-to-roll mapping.
- Provide configurable scoring rules.
- Process batches with bounded memory use.
- Produce reproducible results and useful diagnostic artifacts.
- Reuse the same processing engine in CLI, desktop and web products.

## 6. Functional requirements

### 6.1 Projects and exams

A project represents an exam or evaluation run and contains its template, answer key, candidate mapping, source scans, processing configuration, jobs, results and reports.

### 6.2 Document ingestion

Long-term platform inputs may include PNG, JPG/JPEG, TIFF and PDF. Support single files, batches and ZIP archives. Validate file type, dimensions, readability and basic image quality before processing.

The academic MVP may initially support a narrower set of raster image formats, depending on dataset availability and time.

### 6.3 Layout and template configuration

A template describes page geometry, registration markers, candidate identifier regions, question regions, option positions, sections and supported response types.

Use normalized coordinates where possible so that a template can be applied across different scan resolutions. Apply alignment or perspective correction before extracting regions when required.

### 6.4 Candidate identification

The long-term platform should support:
- bubble-coded numeric identifiers;
- QR codes;
- barcodes;
- printed text where feasible.

The academic MVP should implement the identifier method actually present in the selected dataset/template. Synthetic samples can provide controlled bubble-coded identifiers and roll mappings if the public dataset lacks them.

Candidate identifiers must resolve uniquely. Unknown or duplicate mappings are review conditions, not reasons to guess.

### 6.5 Response recognition

For each question, the system should determine:
- selected option or options;
- blank response;
- multiple-mark response;
- ambiguous response;
- confidence or evidence score;
- diagnostic region information where available.

Question types include single-choice and multiple-select, with configurable option counts in the long-term platform.

### 6.6 Answer keys

The long-term product should support JSON, CSV, image-assisted extraction with human verification, and a manual editor. The academic MVP starts with a documented JSON schema.

### 6.7 Evaluation

Support configurable positive, negative and partial marking. Define explicit policies for blank answers, multiple marks, invalid responses and unanswered questions. Scoring must be deterministic and independently unit-tested.

### 6.8 Results and reporting

Produce structured JSON and CSV results. The future platform may add PDF scorecards, batch analytics, section summaries, review queues and project dashboards.

### 6.9 Human review

The full platform should show the original and aligned image, relevant response region, detected marks, confidence and extracted result. Corrections should be auditable and trigger recalculation.

## 7. Non-functional requirements

- **Correctness:** no silent acceptance of unresolved ambiguity.
- **Performance:** measure per-page latency, throughput and peak memory.
- **Scalability:** bounded batch processing; avoid loading an entire batch into memory.
- **Reliability:** clear errors, safe failure and reproducible processing.
- **Maintainability:** modular code, explicit types and documented interfaces.
- **Portability:** Windows development support for the academic implementation; broader OS support is a later goal.
- **Security:** validate untrusted uploads, avoid unsafe path handling, and isolate project data in the platform.
- **Observability:** stage-level progress, structured errors and diagnostic outputs.

## 8. Constraints

- Academic assignment deadline: approximately 1–2 weeks.
- Academic submission: Rust CLI only.
- Initial academic implementation: no ML.
- Computer vision: Rust with OpenCV bindings.
- Long-term application database preference: PostgreSQL.
- Initial user model: single user, with future multi-user architecture in mind.

## 9. Success criteria

The academic MVP is successful when it can process a documented supported template end-to-end, identify candidates according to the implemented identifier method, extract responses, score them against a supplied answer key, emit results and diagnostics, and report measured accuracy and performance against ground truth.

No numerical accuracy target is asserted until the dataset and evaluation protocol are finalized.

## 10. Out of scope for the academic MVP

GUI, web and desktop clients; PostgreSQL integration; authentication; multi-user access; cloud deployment; ML inference/training; arbitrary-layout universal recognition; and production-grade PDF workflow.


---

<!-- Source: docs/roadmap.md -->

# Product Roadmap

## Phase 0 — Research and technical foundation

1. Compare at least three public OMR datasets.
2. Select an accessible dataset and inspect annotations.
3. Choose and document one initial template.
4. Define synthetic data generation and ground-truth format.
5. Finalize MVP assumptions and acceptance criteria.
6. Validate Rust/OpenCV toolchain on the development machine.
7. Establish baseline project structure and CI.
8. Finalize benchmark methodology.

**Exit condition:** a dataset, template, ground-truth protocol and build environment are selected.

## Phase 1 — Academic Rust CLI (1–2 weeks)

- Build image loading and preprocessing.
- Implement thresholding experiments.
- Implement registration/alignment as required by the template.
- Extract template-based ROIs.
- Implement bubble fill measurement and classification.
- Decode candidate identifiers for the chosen template.
- Implement candidate mapping and scoring.
- Add JSON/CSV outputs and diagnostic overlays.
- Test and benchmark the end-to-end pipeline.
- Write the academic report with measured results.

**Exit condition:** reproducible CLI demonstration and evidence-based report.

## Phase 2 — Core hardening

- Stabilize domain types and interfaces.
- Improve error taxonomy and configuration validation.
- Add more templates and input formats.
- Add robust batch processing and bounded concurrency.
- Add versioned processing configurations and regression fixtures.

## Phase 3 — Local-first platform

- Add PostgreSQL-backed project, template, answer-key, job and result metadata.
- Add filesystem artifact management.
- Build a Rust application/API layer.
- Add job progress and review workflows.
- Add React/TypeScript web UI and Tauri desktop shell over the shared core.

## Phase 4 — Advanced capabilities

- Assisted template configuration.
- QR/barcode and additional candidate-ID methods.
- PDF and bulk archive workflows.
- Rich analytics and PDF reports.
- Optional ML subsystem for difficult layouts and ambiguous marks.

## Phase 5 — Optional cloud deployment and multi-user

- Authentication and authorization.
- Tenant/project isolation.
- Cloud object storage and managed PostgreSQL.
- Deployment automation, monitoring, backups and operational controls.

## Schedule protection

The academic assignment takes priority. Do not start GUI, database, ML or cloud work until the CLI has a tested end-to-end path.


---

<!-- Source: docs/testing-strategy.md -->

# Testing Strategy

## 1. Testing layers

### Unit tests
- coordinate normalization and transform calculations;
- template validation;
- threshold and fill-density functions;
- bubble-state classification;
- candidate-code decoding;
- candidate mapping uniqueness;
- answer-key parsing and validation;
- scoring rules and edge cases;
- JSON/CSV serialization.

### Integration tests
- image decode and preprocessing;
- registration and alignment;
- ROI extraction;
- complete recognition on known fixtures;
- output artifact generation.

### End-to-end tests
Run the CLI against a small fixture manifest and verify candidate IDs, responses, scores, exit codes and output schema.

### Regression tests
Store representative images and expected labels. When algorithms change, compare outputs and require an explicit review of intentional changes.

## 2. Test data

Use:
- a small set of real dataset scans;
- synthetic fixtures with deterministic ground truth;
- edge cases for blank, multiple, faint, heavy and erased marks;
- invalid files and malformed configuration.

Keep test fixtures small; large benchmark datasets should not run in every unit-test invocation.

## 3. Test principles

- Tests must not depend on network access.
- Avoid brittle exact pixel comparisons when a semantic metric is more appropriate.
- Use tolerances for geometric calculations.
- Test failure paths as deliberately as success paths.
- Ensure uncertain inputs produce explicit ambiguous states.
- Test scoring independently from image recognition.
