# Product Requirements Document (PRD)

**Product:** OMNISCAN  
**Version:** 0.1  
**Status:** Draft baseline

## 1. Executive summary

OMNISCAN is an OMR (Optical Mark Recognition) recognition, evaluation and analytics platform. It processes scanned examination sheets, identifies candidates, extracts marked responses, evaluates them against an answer key, and produces structured results and reports.

The long-term product is local-first and supports both web and desktop interfaces over a shared Rust processing core. The academic assignment is deliberately narrower: a Rust CLI that demonstrates a well-engineered, measurable OMR pipeline without machine learning.

## 2. Problem statement

Manual checking of objective examination sheets is repetitive and can be slow and error-prone. An OMR workflow must reliably:

- locate and interpret response bubbles;
- distinguish blank, marked and ambiguous responses;
- identify the candidate;
- apply the correct answer key and marking policy;
- retain enough evidence to audit and correct uncertain cases.

A useful system must handle image quality variation and configurable sheet layouts without pretending that every possible form can be understood automatically.

## 3. Product vision

Build a configurable OMR platform that can process known sheet templates deterministically, assist users in configuring unfamiliar templates, and progressively add automatic layout discovery and optional ML assistance.

### Recognition modes

1. **Known Template Mode:** process a configured layout using deterministic geometry and image analysis.
2. **Assisted Template Mode:** propose detected regions and let an operator confirm or correct them.
3. **Automatic Discovery Mode:** infer candidate layout regions for a previously unseen form; uncertain detections require review.

Universal recognition is not a promise of perfect recognition for arbitrary sheets. Page-size independence is achieved through normalized coordinates and geometric transformations; it does not imply layout independence.

## 4. Target users

- Examination administrators and institutions.
- Teachers and evaluators handling objective tests.
- Small organizations conducting assessments.
- Developers integrating OMR processing into larger systems.

The first release is single-user. The architecture should not block future multi-user operation.

## 5. Product goals

- Correctly interpret supported OMR templates under documented conditions.
- Make recognition uncertainty visible and reviewable.
- Support candidate identification and explicit candidate-to-roll mapping.
- Provide configurable scoring rules.
- Process batches with bounded memory use.
- Produce reproducible results and useful diagnostic artifacts.
- Reuse the same processing engine in CLI, desktop and web products.

## 6. Functional requirements

### 6.1 Projects and exams

A project represents an exam or evaluation run and contains its template, answer key, candidate mapping, source scans, processing configuration, jobs, results and reports.

### 6.2 Document ingestion

Long-term platform inputs may include PNG, JPG/JPEG, TIFF and PDF. Support single files, batches and ZIP archives. Validate file type, dimensions, readability and basic image quality before processing.

The academic MVP may initially support a narrower set of raster image formats, depending on dataset availability and time.

### 6.3 Layout and template configuration

A template describes page geometry, registration markers, candidate identifier regions, question regions, option positions, sections and supported response types.

Use normalized coordinates where possible so that a template can be applied across different scan resolutions. Apply alignment or perspective correction before extracting regions when required.

### 6.4 Candidate identification

The long-term platform should support:
- bubble-coded numeric identifiers;
- QR codes;
- barcodes;
- printed text where feasible.

The academic MVP should implement the identifier method actually present in the selected dataset/template. Synthetic samples can provide controlled bubble-coded identifiers and roll mappings if the public dataset lacks them.

Candidate identifiers must resolve uniquely. Unknown or duplicate mappings are review conditions, not reasons to guess.

### 6.5 Response recognition

For each question, the system should determine:
- selected option or options;
- blank response;
- multiple-mark response;
- ambiguous response;
- confidence or evidence score;
- diagnostic region information where available.

Question types include single-choice and multiple-select, with configurable option counts in the long-term platform.

### 6.6 Answer keys

The long-term product should support JSON, CSV, image-assisted extraction with human verification, and a manual editor. The academic MVP starts with a documented JSON schema.

### 6.7 Evaluation

Support configurable positive, negative and partial marking. Define explicit policies for blank answers, multiple marks, invalid responses and unanswered questions. Scoring must be deterministic and independently unit-tested.

### 6.8 Results and reporting

Produce structured JSON and CSV results. The future platform may add PDF scorecards, batch analytics, section summaries, review queues and project dashboards.

### 6.9 Human review

The full platform should show the original and aligned image, relevant response region, detected marks, confidence and extracted result. Corrections should be auditable and trigger recalculation.

## 7. Non-functional requirements

- **Correctness:** no silent acceptance of unresolved ambiguity.
- **Performance:** measure per-page latency, throughput and peak memory.
- **Scalability:** bounded batch processing; avoid loading an entire batch into memory.
- **Reliability:** clear errors, safe failure and reproducible processing.
- **Maintainability:** modular code, explicit types and documented interfaces.
- **Portability:** Windows development support for the academic implementation; broader OS support is a later goal.
- **Security:** validate untrusted uploads, avoid unsafe path handling, and isolate project data in the platform.
- **Observability:** stage-level progress, structured errors and diagnostic outputs.

## 8. Constraints

- Academic assignment deadline: approximately 1–2 weeks.
- Academic submission: Rust CLI only.
- Initial academic implementation: no ML.
- Computer vision: Rust with OpenCV bindings.
- Long-term application database preference: PostgreSQL.
- Initial user model: single user, with future multi-user architecture in mind.

## 9. Success criteria

The academic MVP is successful when it can process a documented supported template end-to-end, identify candidates according to the implemented identifier method, extract responses, score them against a supplied answer key, emit results and diagnostics, and report measured accuracy and performance against ground truth.

No numerical accuracy target is asserted until the dataset and evaluation protocol are finalized.

## 10. Out of scope for the academic MVP

GUI, web and desktop clients; PostgreSQL integration; authentication; multi-user access; cloud deployment; ML inference/training; arbitrary-layout universal recognition; and production-grade PDF workflow.
