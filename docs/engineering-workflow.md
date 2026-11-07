# Engineering Workflow and Quality Standards

## 1. Repository principles

- Keep modules cohesive and dependencies directed inward.
- Use explicit domain types instead of passing loosely structured values.
- Keep OpenCV-specific types behind adapters.
- Keep configuration external and validated.
- Preserve original input files.
- Prefer small, reviewable commits.

## 2. Suggested repository structure

```text
omniscan/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── configs/
├── datasets/
│   ├── public/
│   ├── synthetic/
│   ├── answer_keys/
│   └── mappings/
├── docs/
├── src/
│   ├── main.rs
│   ├── cli/
│   ├── core/
│   ├── imaging/
│   ├── layout/
│   ├── recognition/
│   ├── evaluation/
│   ├── pipeline/
│   ├── output/
│   └── benchmark/
├── tests/
└── scripts/
```

Create files incrementally as implementation requires; the tree is a target organization, not a requirement to create empty modules immediately.

## 3. Rust quality

- Run `cargo fmt --check`.
- Run `cargo clippy --all-targets --all-features -- -D warnings` where practical.
- Run `cargo test`.
- Document public types and non-obvious algorithms.
- Avoid unnecessary cloning and unbounded allocations.
- Use `Result` for recoverable errors; avoid panics on user input.
- Validate all external configuration and file paths.

## 4. Git workflow

Use focused conventional commits, for example:

```text
docs: establish OMNISCAN product and engineering baseline
chore: initialize Rust workspace and CI checks
feat(imaging): add validated image loading
feat(layout): add normalized template geometry
feat(recognition): classify bubble response states
feat(evaluation): implement configurable scoring
test(pipeline): add end-to-end OMR fixtures
perf(batch): add bounded parallel processing
```

## 5. Definition of done

A change is done when:
- behavior and assumptions are documented;
- relevant tests pass;
- invalid inputs and failure modes are handled;
- output is deterministic where expected;
- performance-sensitive changes have measurements;
- no original input is overwritten;
- logs and diagnostics are useful;
- the change is committed with a focused message.

## 6. CI/CD direction

For the academic MVP, CI should build and test the Rust project and run formatting/lint checks. Later platform phases can add frontend checks, integration environments, security scanning, packaging and deployment workflows.
