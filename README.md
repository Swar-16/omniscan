# OMNISCAN

A local-first, configurable Optical Mark Recognition (OMR) platform. The first deliverable is a deterministic Rust CLI academic MVP; the roadmap later adds shared-core web and desktop applications.

## Status

**Phase 0 — research and technical foundation.**
- **M0 (Rust Project and Environment Foundation):** Implemented.
  - Rust project initialized with `clap` and `thiserror`.
  - Pinned stable MSVC toolchain via `rust-toolchain.toml`.
  - Typed error hierarchy and discoverable CLI `--help`.
  - CI workflow configured for formatting, Clippy, and tests.
  - Windows environment and OpenCV build prerequisites documented in [`docs/windows-setup.md`](docs/windows-setup.md).
- **M1 (Dataset Inventory and Template Configuration):** Implemented.
  - Normalized 2D geometric domain types (`Point2D`, `Circle`, `NormalizedRect`).
  - Versioned OMR JSON template representation and strict geometric validator.
  - Calibrated synthetic 30-question reference template (`configs/templates/synthetic-standard-30q-v1.json`).
  - Versioned dataset manifest schema with path-traversal protection.
  - Multi-delimiter ground-truth label parser with line-number diagnostics.
  - Functional `omniscan inspect-template` CLI command.
  - Documented Tamaulipas dataset profile and reconciliation in [`docs/datasets/tamaulipas-research.md`](docs/datasets/tamaulipas-research.md).

## Read first

1. [`AGENTS.md`](AGENTS.md) — mandatory instructions for coding agents.
2. [`implemented.md`](implemented.md) — agent implementation and operational ledger.
3. [`docs/00-project-charter.md`](docs/00-project-charter.md) — product decisions and scope.
4. [`docs/01-dataset-decision.md`](docs/01-dataset-decision.md) — dataset comparison and preflight.
5. [`docs/datasets/tamaulipas-research.md`](docs/datasets/tamaulipas-research.md) — Tamaulipas dataset profile and reconciliation.
6. [`docs/02-implementation-plan.md`](docs/02-implementation-plan.md) — milestone plan and acceptance criteria.
7. [`docs/06-ai-agent-execution-playbook.md`](docs/06-ai-agent-execution-playbook.md) — how to delegate tasks to an AI coding agent.
8. [`docs/windows-setup.md`](docs/windows-setup.md) — Windows development setup and native OpenCV build prerequisites.
9. [`docs/adr/README.md`](docs/adr/README.md) — architecture decisions.

## Principles

- Deterministic, explainable recognition for supported templates.
- Explicit ambiguity instead of silent guesses.
- Original scans are preserved.
- Template and processing configuration are versioned.
- Accuracy and performance are measured, never invented.
- No ML, database, GUI or cloud dependency in the academic MVP.

## Dataset policy

Dataset downloads are separate from build and test. Review the license and dataset preflight before downloading or redistributing files. Large datasets do not belong in Git.

## Development and Verification

### Prerequisites
- Stable Rust toolchain (`stable-x86_64-pc-windows-msvc`) with `rustfmt` and `clippy`.
- MSVC C++ Build Tools & LLVM Clang (see [`docs/windows-setup.md`](docs/windows-setup.md)).

### Quality Commands

Run from the `omniscan` directory:

```bash
# Code formatting check
cargo fmt --check

# Clippy linter with zero warnings tolerance
cargo clippy --all-targets --all-features -- -D warnings

# Unit and integration test suite
cargo test
```

### CLI Usage

```bash
# Display help and subcommands
cargo run -- --help

# Inspect and validate an OMR sheet template
cargo run -- inspect-template configs/templates/synthetic-standard-30q-v1.json

# Validate an image scan stub (M2)
cargo run -- validate-image scan.png

# Process a single scan stub (M7)
cargo run -- process scan.png --template template.json --answer-key key.json
```

## License

This project is licensed under the [MIT License](LICENSE).
