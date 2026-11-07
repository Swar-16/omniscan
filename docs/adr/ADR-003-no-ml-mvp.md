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
