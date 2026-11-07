# Testing Strategy

## 1. Testing layers

### Unit tests
- coordinate normalization and transform calculations;
- template validation;
- threshold and fill-density functions;
- bubble-state classification;
- candidate-code decoding;
- candidate mapping uniqueness;
- answer-key parsing and validation;
- scoring rules and edge cases;
- JSON/CSV serialization.

### Integration tests
- image decode and preprocessing;
- registration and alignment;
- ROI extraction;
- complete recognition on known fixtures;
- output artifact generation.

### End-to-end tests
Run the CLI against a small fixture manifest and verify candidate IDs, responses, scores, exit codes and output schema.

### Regression tests
Store representative images and expected labels. When algorithms change, compare outputs and require an explicit review of intentional changes.

## 2. Test data

Use:
- a small set of real dataset scans;
- synthetic fixtures with deterministic ground truth;
- edge cases for blank, multiple, faint, heavy and erased marks;
- invalid files and malformed configuration.

Keep test fixtures small; large benchmark datasets should not run in every unit-test invocation.

## 3. Test principles

- Tests must not depend on network access.
- Avoid brittle exact pixel comparisons when a semantic metric is more appropriate.
- Use tolerances for geometric calculations.
- Test failure paths as deliberately as success paths.
- Ensure uncertain inputs produce explicit ambiguous states.
- Test scoring independently from image recognition.
