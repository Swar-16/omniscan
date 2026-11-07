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
