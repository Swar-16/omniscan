# OMNISCAN

A local-first, configurable Optical Mark Recognition (OMR) platform. The first deliverable is a deterministic Rust CLI academic MVP; the roadmap later adds shared-core web and desktop applications.

## Status

**Phase 0 — research and technical foundation.** Dataset/template preflight and Rust/OpenCV environment validation are the immediate tasks.

## Read first

1. [`AGENTS.md`](AGENTS.md) — mandatory instructions for coding agents.
2. [`docs/00-project-charter.md`](docs/00-project-charter.md) — product decisions and scope.
3. [`docs/01-dataset-decision.md`](docs/01-dataset-decision.md) — dataset comparison and preflight.
4. [`docs/02-implementation-plan.md`](docs/02-implementation-plan.md) — milestone plan and acceptance criteria.
5. [`docs/06-ai-agent-execution-playbook.md`](docs/06-ai-agent-execution-playbook.md) — how to delegate tasks to an AI coding agent.
6. [`docs/adr/README.md`](docs/adr/README.md) — architecture decisions.

## Principles

- Deterministic, explainable recognition for supported templates.
- Explicit ambiguity instead of silent guesses.
- Original scans are preserved.
- Template and processing configuration are versioned.
- Accuracy and performance are measured, never invented.
- No ML, database, GUI or cloud dependency in the academic MVP.

## Dataset policy

Dataset downloads are separate from build and test. Review the license and dataset preflight before downloading or redistributing files. Large datasets do not belong in Git.

## Development

Exact build instructions will be finalized after the Rust/OpenCV environment check. Expected quality commands:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## License

To be selected deliberately before public release.
