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
