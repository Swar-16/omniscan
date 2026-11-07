# GitHub Repository Setup and Workflow

## 1. Create repository

Suggested repository name: `omniscan`.

Suggested description:
> A local-first, configurable Optical Mark Recognition platform with a high-performance Rust processing core, CLI, and planned web and desktop applications.

Create the repository without committing a dataset. A public repository is appropriate for portfolio visibility if you are comfortable sharing the source; choose private if you want to delay public release.

## 2. First local setup

```bash
git clone https://github.com/<YOUR_USERNAME>/omniscan.git
cd omniscan
git switch -c docs/initial-project-foundation
```

If you created the repository without a README, create the first files locally instead. Avoid unrelated histories by deciding on one initialization path.

## 3. Branch conventions

- `main`: stable, tested state.
- `docs/<topic>`: documentation-only changes.
- `feat/<topic>`: feature implementation.
- `fix/<topic>`: bug fixes.
- `test/<topic>`: test-only work where useful.

Keep branches short-lived and focused.

## 4. Commit examples

```text
docs: establish OMNISCAN product and engineering baseline
chore: initialize Rust project and CI checks
feat(imaging): add validated image loading
feat(layout): add normalized template geometry
feat(recognition): classify bubble response states
feat(evaluation): implement configurable scoring
test(pipeline): add end-to-end OMR fixtures
perf(batch): add bounded parallel processing
```

## 5. Initial GitHub settings

- Protect `main` against force pushes and accidental deletion.
- Require CI checks before merging when practical.
- Enable Issues and Discussions only if you plan to use them.
- Add issue templates for bug reports and feature requests once the code exists.
- Use GitHub Actions for format, lint and test checks.
- Add a license only after choosing one deliberately.
- Do not enable Git LFS or commit large data by default. Store dataset download instructions and checksums in documentation instead.

## 6. Dataset policy

- Keep small, synthetic, non-sensitive test fixtures in the repository.
- Store large datasets outside Git and download them through a documented manual or opt-in script.
- Record dataset version, license, source URL and checksum.
- Never commit student-identifying data.
- Do not redistribute a dataset unless its license permits it.

## 7. First milestones as Issues

Create one issue per milestone from `docs/02-implementation-plan.md`. Each issue must state scope, dependencies, acceptance criteria and explicit out-of-scope items. Ask the agent to work on one issue at a time.
