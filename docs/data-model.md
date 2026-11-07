# Conceptual Data Model

This document describes the future platform's conceptual entities. The academic CLI may represent these as in-memory Rust structs and JSON configuration rather than database tables.

## 1. Entities

### Project
An exam/evaluation workspace. Has an identifier, name, description, status and timestamps.

### Template
A versioned description of page geometry, registration features, candidate-ID regions, question regions, option locations and supported response types.

### AnswerKey
A versioned set of expected answers and scoring rules associated with an exam/project.

### Candidate
A person or exam participant. Contains an internal identifier and permitted identifying attributes.

### CandidateCodeMapping
Maps a code read from the sheet to exactly one candidate/roll number. Enforce uniqueness where required.

### Scan
A source document and its metadata: file reference, checksum, dimensions, upload time, detected quality and processing status.

### ProcessingJob
Tracks a batch or single-scan run, configuration version, status, timestamps, progress and errors.

### RecognitionResult
Stores candidate identification, per-question response states, confidence/evidence and processing provenance.

### EvaluationResult
Stores awarded marks, maximum marks, question-level breakdown and scoring-policy version.

### ReviewAction
Records a human correction or confirmation, actor, timestamp, old value, new value and reason.

### Artifact
References original scans, aligned images, overlays, annotated images, exports and reports.

## 2. Relationship sketch

```text
Project 1---* TemplateVersion
Project 1---* AnswerKeyVersion
Project 1---* Scan
Project 1---* ProcessingJob
Project 1---* Candidate
Candidate 1---* CandidateCodeMapping
Scan 1---* RecognitionResult
ProcessingJob 1---* RecognitionResult
RecognitionResult 1---1 EvaluationResult
RecognitionResult 1---* ReviewAction
Scan 1---* Artifact
```

## 3. Persistence principles

- PostgreSQL stores structured metadata and transactional state.
- Binary image/report artifacts live in filesystem or object storage.
- Store checksums and artifact references for integrity and traceability.
- Version templates, answer keys and processing configurations.
- Preserve the exact configuration and software version used for each processing run.
- Do not overwrite original scans during preprocessing.
