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

## Read first

1. [`AGENTS.md`](AGENTS.md) — mandatory instructions for coding agents.
2. [`implemented.md`](implemented.md) — agent implementation and operational ledger.
3. [`docs/00-project-charter.md`](docs/00-project-charter.md) — product decisions and scope.
4. [`docs/01-dataset-decision.md`](docs/01-dataset-decision.md) — dataset comparison and preflight.
5. [`docs/02-implementation-plan.md`](docs/02-implementation-plan.md) — milestone plan and acceptance criteria.
6. [`docs/06-ai-agent-execution-playbook.md`](docs/06-ai-agent-execution-playbook.md) — how to delegate tasks to an AI coding agent.
7. [`docs/windows-setup.md`](docs/windows-setup.md) — Windows development setup and native OpenCV build prerequisites.
8. [`docs/adr/README.md`](docs/adr/README.md) — architecture decisions.

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

# Validate an image scan stub
cargo run -- validate-image scan.png

# Inspect a template definition stub
cargo run -- inspect-template configs/templates/initial-template.json

# Process a single scan
cargo run -- process scan.png --template template.json --answer-key key.json
```

## License

This project is licensed under the [MIT License](LICENSE).
