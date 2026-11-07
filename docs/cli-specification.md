# CLI Specification (Proposed)

The exact command names may change during implementation. Keep the CLI discoverable, composable and script-friendly.

## 1. Proposed commands

```text
omniscan
  validate-image <IMAGE>
  inspect-template <TEMPLATE.json>
  process <IMAGE> --template <TEMPLATE.json> --answer-key <KEY.json>
  batch <INPUT_DIR> --template <TEMPLATE.json> --answer-key <KEY.json>
  evaluate <RESPONSES.json> --answer-key <KEY.json>
  benchmark <MANIFEST.json> --template <TEMPLATE.json>
```

## 2. Output conventions

- Machine-readable JSON for structured results.
- CSV for tabular batch results.
- Diagnostic images written to an explicit output directory.
- Progress and human-readable diagnostics to stderr.
- Avoid mixing progress text into JSON stdout.
- Return non-zero exit codes for invalid input, configuration errors and processing failures.

## 3. Configuration

Keep template, preprocessing and recognition configuration separate where useful:

```text
configs/
  templates/
    initial-template.json
  preprocessing.json
  recognition.json
```

Every run should record the configuration version or a checksum so results can be reproduced.

## 4. Error handling

Errors should identify the stage and affected file. Distinguish:
- invalid or unsupported file;
- image decode failure;
- invalid template;
- alignment failure;
- recognition ambiguity;
- candidate mapping failure;
- answer-key validation failure;
- output write failure.

Ambiguity is not necessarily a fatal process error: it may produce a result requiring review.
