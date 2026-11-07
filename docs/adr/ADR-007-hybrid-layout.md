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
