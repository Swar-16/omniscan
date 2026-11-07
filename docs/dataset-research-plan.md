# Dataset Research and Selection Plan

**Status: Pending research.** No dataset is selected in this document.

## 1. Goal

Choose a public scanned OMR dataset suitable for a short, measurable Rust CLI assignment. Dataset selection determines the initial template, ground-truth format, recognition approach and evaluation claims.

## 2. Minimum comparison set

Investigate at least three candidate public datasets or reproducible research resources. For each, record:

| Criterion | Questions |
|---|---|
| Access | Is there a direct download or a documented request process? |
| License | Is academic use permitted? Are redistribution restrictions clear? |
| Images | Are actual scanned/photo OMR sheets available? |
| Ground truth | Are marked answers labelled per question? |
| Template | Is the sheet geometry documented or inferable? |
| Candidate ID | Is an identifier present and labelled? |
| Scale | How many sheets and pages are available? |
| Variation | Does it include rotation, skew, lighting, noise and print variation? |
| Format | Image/PDF format, resolution and color depth? |
| Reproducibility | Are scripts, paper, annotation schema or benchmark splits available? |

## 3. Selection principles

Prefer a dataset that:
1. can be accessed immediately;
2. includes real scanned sheets;
3. has trustworthy answer-level ground truth;
4. uses a consistent, documented template;
5. is small enough to inspect and process within the deadline;
6. has clear academic usage terms.

A dataset without candidate identifiers can still support answer recognition. In that case, use synthetic sheets for candidate-ID and roll-mapping tests and disclose the split between real and synthetic evaluation.

## 4. Synthetic dataset strategy

Build a generator only after documenting the selected template. Generate known answer patterns and candidate codes, then render bubbles and introduce controlled variations:
- translation, rotation and mild perspective;
- scale and resolution changes;
- blur and sensor noise;
- brightness/contrast changes;
- faint and heavy marks;
- erasures and stray marks;
- multiple marks and blank responses.

Store ground truth beside each generated image in a machine-readable manifest. Use deterministic random seeds. Keep training/tuning samples separate from held-out evaluation samples.

Synthetic data is useful for controlled tests, but it must not be presented as a substitute for real-world validation.

## 5. Deliverables

- Dataset comparison table with URLs, access date and license notes.
- Chosen dataset and rationale.
- Sample inventory and annotation audit.
- Template geometry document.
- Ground-truth schema.
- Synthetic generation specification.
- Train/validation/test split manifest.

## 6. Important limitation

Do not claim that a dataset supports candidate identification, multiple layouts or arbitrary page sizes unless its actual files and labels demonstrate those properties.
