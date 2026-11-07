# ADR-004: Start With a Modular Monolith

- **Status:** Accepted

## Context
The product is early-stage and the academic deliverable is a single CLI. Distributed services would add operational complexity before there is evidence they are needed.

## Decision
Use a modular monolith with clear module boundaries.

## Consequences
- Easier local development, testing and deployment.
- Modules can later be extracted if scale or independent deployment justifies it.
- Enforce dependency direction and avoid cross-module implementation leakage.
