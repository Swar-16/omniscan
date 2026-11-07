# Product Roadmap

## Phase 0 — Research and technical foundation

1. Compare at least three public OMR datasets.
2. Select an accessible dataset and inspect annotations.
3. Choose and document one initial template.
4. Define synthetic data generation and ground-truth format.
5. Finalize MVP assumptions and acceptance criteria.
6. Validate Rust/OpenCV toolchain on the development machine.
7. Establish baseline project structure and CI.
8. Finalize benchmark methodology.

**Exit condition:** a dataset, template, ground-truth protocol and build environment are selected.

## Phase 1 — Academic Rust CLI (1–2 weeks)

- Build image loading and preprocessing.
- Implement thresholding experiments.
- Implement registration/alignment as required by the template.
- Extract template-based ROIs.
- Implement bubble fill measurement and classification.
- Decode candidate identifiers for the chosen template.
- Implement candidate mapping and scoring.
- Add JSON/CSV outputs and diagnostic overlays.
- Test and benchmark the end-to-end pipeline.
- Write the academic report with measured results.

**Exit condition:** reproducible CLI demonstration and evidence-based report.

## Phase 2 — Core hardening

- Stabilize domain types and interfaces.
- Improve error taxonomy and configuration validation.
- Add more templates and input formats.
- Add robust batch processing and bounded concurrency.
- Add versioned processing configurations and regression fixtures.

## Phase 3 — Local-first platform

- Add PostgreSQL-backed project, template, answer-key, job and result metadata.
- Add filesystem artifact management.
- Build a Rust application/API layer.
- Add job progress and review workflows.
- Add React/TypeScript web UI and Tauri desktop shell over the shared core.

## Phase 4 — Advanced capabilities

- Assisted template configuration.
- QR/barcode and additional candidate-ID methods.
- PDF and bulk archive workflows.
- Rich analytics and PDF reports.
- Optional ML subsystem for difficult layouts and ambiguous marks.

## Phase 5 — Optional cloud deployment and multi-user

- Authentication and authorization.
- Tenant/project isolation.
- Cloud object storage and managed PostgreSQL.
- Deployment automation, monitoring, backups and operational controls.

## Schedule protection

The academic assignment takes priority. Do not start GUI, database, ML or cloud work until the CLI has a tested end-to-end path.
