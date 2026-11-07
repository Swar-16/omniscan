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
