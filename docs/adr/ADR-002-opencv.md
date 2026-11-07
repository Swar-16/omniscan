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
