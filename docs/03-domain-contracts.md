# Domain Contracts and Data Shapes

These are initial contracts for discussion and implementation. Treat them as proposed stable boundaries, not permission to invent unsupported fields. Update the document alongside implementation.

## Response state

Conceptually, each question response must distinguish:
- `selected`: one recognized option;
- `blank`: no mark exceeds the fill criterion;
- `multiple`: more than one option exceeds the mark criterion;
- `ambiguous`: evidence is insufficient or conflicting.

Do not encode all these states as a single option index or a magic integer.

## Question response

Proposed conceptual fields:
- question identifier/index;
- response state;
- selected option(s), when applicable;
- per-option fill measurements;
- optional confidence/evidence;
- review-required flag and reason.

## Candidate identity

Proposed states:
- `resolved`: identifier maps uniquely;
- `unresolved`: no identifier could be decoded;
- `unknown_code`: decoded code absent from mapping;
- `ambiguous`: multiple possible identifier values;
- `duplicate_mapping`: mapping configuration is invalid.

## Processing result

A result should record:
- source reference/path;
- template identifier and version;
- processing configuration identifier/checksum;
- candidate identity result;
- ordered question responses;
- evaluation result if a key was supplied;
- warnings/errors;
- paths or references to diagnostic artifacts;
- software version when available.

## JSON contract rules

- Use explicit strings for enum-like states.
- Keep question ordering explicit and stable.
- Reject unknown or missing required configuration fields where safety/correctness demands it.
- Include a schema version for persisted output.
- Never include absolute local paths in shared examples unless clearly marked as machine-specific.
- Avoid personally identifying candidate data in test fixtures.
- Document whether scores are integers, decimal values or rational calculations.

## Initial answer key example

```json
{
  "schema_version": 1,
  "key_id": "demo-key-v1",
  "questions": [
    {
      "question_id": "Q1",
      "correct_options": ["B"],
      "marks_correct": 4.0,
      "marks_incorrect": -1.0,
      "marks_blank": 0.0
    },
    {
      "question_id": "Q2",
      "correct_options": ["A", "C"],
      "marks_correct": 4.0,
      "marks_incorrect": -1.0,
      "marks_blank": 0.0
    }
  ]
}
```

The example illustrates a possible schema, not a requirement to implement multi-select in the first vertical slice. The implementation must decide and document how partial credit is calculated for multi-select responses before enabling it.
