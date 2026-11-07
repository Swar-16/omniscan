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
