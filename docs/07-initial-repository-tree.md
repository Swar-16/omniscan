# Initial Repository Tree

Use this as a target structure. Create directories when they contain real code; avoid empty scaffolding for every future module.

```text
omniscan/
├── AGENTS.md
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE                  # only after license decision
├── .gitignore
├── rust-toolchain.toml      # if a pinned toolchain is chosen
├── configs/
│   ├── templates/
│   ├── preprocessing.json
│   └── recognition.json
├── docs/
│   ├── 00-project-charter.md
│   ├── 01-dataset-decision.md
│   ├── 02-implementation-plan.md
│   ├── 03-domain-contracts.md
│   ├── 04-testing-and-benchmark-contract.md
│   ├── 05-github-workflow.md
│   ├── 06-ai-agent-execution-playbook.md
│   ├── 07-initial-repository-tree.md
│   └── adr/
├── examples/
│   ├── answer-keys/
│   └── mappings/
├── scripts/
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
│   ├── fixtures/
│   └── end_to_end.rs
└── .github/
    └── workflows/
        └── ci.yml
```

## Dataset location

Do not put a large dataset in the repository. A local directory such as `data/external/` may be gitignored. Commit only small synthetic fixtures whose provenance and generated ground truth are documented.
