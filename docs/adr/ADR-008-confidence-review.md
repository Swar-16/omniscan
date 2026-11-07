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
