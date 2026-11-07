# AI Agent Execution Playbook

## Goal

Use an AI coding agent as an implementation partner without allowing it to silently expand scope or invent architecture.

## Operating loop

1. Give the agent `AGENTS.md` and the relevant document(s).
2. Ask it to inspect the repository and report current state before editing.
3. Assign exactly one milestone or small task.
4. Require a proposed plan and acceptance checklist.
5. Approve or correct the plan.
6. Let it implement and test.
7. Review the diff, tests and documentation.
8. Merge only when acceptance criteria pass.

## First prompt: repository reconnaissance

```text
Read AGENTS.md and docs/00-project-charter.md through
docs/06-ai-agent-execution-playbook.md, plus the ADRs.

Do not implement features yet. Inspect the repository and report:
1. current repository state;
2. missing files and environment prerequisites;
3. proposed M0 plan;
4. OpenCV/Rust risks on this machine;
5. commands you would run to validate M0.

Do not change files, install packages, download datasets, or make commits
until I approve the plan.
```

## M0 implementation prompt

```text
Implement only M0 from docs/02-implementation-plan.md.

First inspect the repository and propose the smallest implementation plan.
Follow AGENTS.md and existing ADRs. Initialize the minimal Rust CLI, add
typed top-level errors and a `--help` command, and configure CI for fmt,
clippy and tests. Do not implement OMR algorithms, do not download datasets,
do not add a database, GUI, ML or cloud services.

Acceptance:
- clean-checkout build and tests work;
- formatting/lint/test commands are documented;
- CI configuration is present;
- no unrelated refactors;
- report exact commands and their results.
Wait for approval before broadening scope.
```

## Feature task prompt template

```text
Task: <single task title>

Read AGENTS.md and these documents:
- <relevant docs>
- <relevant ADRs>

Before editing:
1. inspect the relevant code;
2. state the intended design and files;
3. list acceptance criteria and tests;
4. identify assumptions or questions.

Implement only this task. Do not expand scope or change accepted ADRs.
Add tests, update docs and run the applicable quality checks.
Report files changed, commands/results, limitations and next task.
Do not commit or push unless explicitly instructed.
```

## Review prompt

```text
Review this diff against AGENTS.md, the task acceptance criteria and relevant ADRs.
Do not edit code yet. Identify correctness bugs, algorithmic weaknesses,
missing tests, API/schema inconsistencies, unnecessary dependencies,
memory risks, licensing concerns and unsupported claims. Rank findings
by severity and point to exact files/lines. Distinguish blockers from suggestions.
```

## Rules for agent selection

Choose an agent that can inspect/edit repository files and run terminal commands. Give it the entire repository context or point it to `AGENTS.md`; do not rely on a one-time chat prompt that is lost between tasks. If the platform supports repository-level instruction files, keep `AGENTS.md` at the root.

## Human review checklist

- Does the diff implement only the assigned task?
- Are the algorithms explained and testable?
- Are uncertain inputs handled explicitly?
- Are dependencies justified?
- Are test outputs real and reproducible?
- Are documentation and actual behavior consistent?
- Has any data been downloaded or uploaded without permission?
- Are there unapproved schema/ADR changes?
