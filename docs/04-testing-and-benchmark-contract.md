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
