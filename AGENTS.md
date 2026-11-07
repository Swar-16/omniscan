# AGENTS.md — OMNISCAN Repository Operating Contract

This file is the mandatory instruction set for any AI coding agent working in this repository. Read it before changing files.

## 1. Mission

Build OMNISCAN: a configurable Optical Mark Recognition (OMR) system. The first deliverable is a reliable, explainable Rust CLI academic MVP. The long-term direction is a local-first platform with web and desktop interfaces sharing the same Rust processing core.

## 2. Current phase and priority

The current phase is **Phase 0: research and foundation** unless the repository owner explicitly changes it.

Priority order:
1. Confirm dataset access, license and label schema.
2. Select and document one template for the first end-to-end pipeline.
3. Verify Rust/OpenCV build environment.
4. Establish minimal Rust project, tests and CI.
5. Implement one thin end-to-end vertical slice.
6. Expand algorithms only after the slice works.

Do not start GUI, database, authentication, cloud deployment or ML work while the academic CLI is incomplete.

## 3. Non-negotiable constraints

- Rust is the authoritative implementation language for the academic OMR core.
- Use OpenCV through Rust bindings for computer-vision primitives unless a written ADR changes this.
- No machine learning in the academic MVP.
- The MVP is a CLI, not a web or desktop application.
- The MVP must work without PostgreSQL or network access at runtime.
- Do not invent dataset contents, labels, accuracy, performance measurements or test results.
- Do not claim support for layouts or identifiers that have not been implemented and tested.
- Preserve original scans; derived images are separate artifacts.
- Ambiguous responses must remain explicitly ambiguous/review-required. Never silently guess.
- Do not add dependencies without explaining their purpose and maintenance/build impact.
- Do not download large datasets automatically as part of build/test.
- Do not commit dataset files unless the repository owner has explicitly approved it and licensing/size have been checked.
- Do not add a second database such as SQLite as a shortcut; PostgreSQL is a future-platform direction, while the academic CLI is file-based.
- Do not use unsafe Rust unless justified, isolated and documented.
- Do not make unrelated broad refactors.

## 4. Required workflow for every task

1. Read this file and the relevant documents under `docs/`.
2. Inspect the current code and tests before proposing changes.
3. State the intended change, affected modules, risks and acceptance criteria.
4. Implement the smallest coherent increment.
5. Add or update tests.
6. Run applicable checks and report exact commands/results. If a command could not run, say why.
7. Update documentation when behavior, configuration, output or assumptions change.
8. Summarize changed files, decisions, known limitations and next task.
9. Do not mark a task complete if its acceptance criteria are unmet.

## 5. Git behavior

- Never force-push, rewrite shared history, delete branches, or push without explicit user permission.
- Work on a task-specific branch when the environment supports Git branches.
- Prefer focused Conventional Commits, e.g. `feat(imaging): add validated image loading`.
- Never commit secrets, personal data, licensed data without permission, build artifacts, or large generated images.
- Do not create a commit unless requested or the agent environment's task explicitly authorizes commits.

## 6. Rust quality gate

Run, when applicable:
- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test`

For Windows/OpenCV build problems, report the exact toolchain, environment variables and error. Do not silently replace OpenCV with a different stack.

## 7. Architecture rules

- Keep domain types and scoring independent of OpenCV.
- Confine OpenCV `Mat` and binding-specific details to imaging adapters where practical.
- Keep `evaluation` independent of image recognition.
- Keep template geometry and recognition thresholds in validated configuration, not scattered magic constants.
- Use typed errors rather than panics for expected input failures.
- Bound memory and concurrency for batch work.
- Make output reproducible for identical inputs, configuration and software version where practical.
- Version templates, answer keys and processing configuration.
- Use `HashMap` for candidate-code lookup and `Vec` for ordered question responses unless measurements justify another structure.
- Don't introduce elaborate abstractions without a concrete use case.

## 8. Definition of done

A task is done only when:
- acceptance criteria are met;
- tests are added or updated and pass, or failures are explicitly explained;
- errors and invalid inputs are handled;
- docs and examples match actual behavior;
- no unsupported claims are made;
- limitations are listed;
- the final response gives commands run and their results.

## 9. Stop and ask the owner

Ask before:
- changing an accepted ADR;
- adding ML to the academic MVP;
- adding a database dependency to the academic CLI;
- committing or redistributing a dataset;
- changing public JSON/CSV output schemas incompatibly;
- deleting or rewriting user data;
- making a major scope or licensing decision.

## 10. Agent response format

For each task, report:
1. **Implemented**
2. **Design decisions**
3. **Files changed**
4. **Validation commands and results**
5. **Known limitations**
6. **Next smallest task**
