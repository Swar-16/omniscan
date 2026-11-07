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
