# System Architecture

## 1. Architectural direction

Use a **modular monolith** initially. The Rust processing core is shared by the CLI and future application interfaces. Avoid microservices until real deployment or scaling requirements justify them.

## 2. Target architecture

```text
                    User Interfaces
             +----------+----------+
             |          |          |
            CLI       Desktop      Web
                        |           |
                     Tauri       React/TS
                        \           /
                         API / App Layer
                               |
                      Shared Rust Core
       +-----------------------+-----------------------+
       |                       |                       |
   Ingestion               OMR Engine              Evaluation
       |                       |                       |
   Validation       Preprocess -> Align -> ROI     Answer Key
   File handling    Layout -> Bubble recognition  Scoring
                    Candidate identification       Results
       +-----------------------+-----------------------+
                               |
                Persistence / Artifact Storage
                 PostgreSQL + File/Object Storage
                               |
                 Optional Intelligence Subsystem
                  Python ML experiments/services
```

The academic MVP implements the Rust core and CLI path only.

## 3. Core modules

- `core`: shared domain types, errors, configuration and traits.
- `imaging`: loading, preprocessing, thresholding and geometry.
- `layout`: templates, regions and registration features.
- `recognition`: bubble analysis, identifier extraction and confidence.
- `evaluation`: answer keys, candidate mapping and scoring.
- `pipeline`: orchestration of the processing stages.
- `output`: JSON/CSV serialization and annotated images.
- `benchmark`: metrics and experiment reporting.
- `cli`: argument parsing and command handlers.

## 4. Dependency direction

The domain/core layer should not depend on OpenCV. OpenCV-specific types should be confined to imaging adapters. Recognition and evaluation should consume project-owned types rather than exposing `opencv::core::Mat` throughout the codebase.

Example abstraction:

```rust
pub trait ImagePreprocessor {
    fn preprocess(
        &self,
        input: &ImageFrame,
    ) -> Result<ProcessedImage, ProcessingError>;
}

pub trait BubbleRecognizer {
    fn recognize(
        &self,
        image: &ProcessedImage,
        layout: &OmrLayout,
    ) -> Result<Vec<QuestionResponse>, RecognitionError>;
}
```

These interfaces are illustrative; refine them after the selected dataset and template are known.

## 5. Storage boundaries in the future platform

PostgreSQL is the canonical store for structured application data: projects, templates, answer-key versions, jobs, candidate mappings, results, review actions and audit events.

Store large binary artifacts—original scans, normalized images, annotations and generated reports—in filesystem or object storage. Store stable references and metadata in PostgreSQL rather than placing all image bytes in relational rows.

The local-first PostgreSQL lifecycle remains an ADR to validate. The academic CLI should not depend on a database.

## 6. Concurrency and memory

Use bounded work queues for batch processing. Parallelize independent page-level work only after measuring memory per in-flight image. A configurable worker count should be available. Do not read an unbounded batch into RAM.

## 7. Future ML boundary

Keep ML optional and isolated behind a stable interface. Python may be used for training and experimentation; the deterministic Rust pipeline remains the default. Any ML-assisted output must expose confidence and preserve human review for uncertain cases.
