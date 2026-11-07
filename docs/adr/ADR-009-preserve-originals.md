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
