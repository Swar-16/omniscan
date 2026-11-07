# Dataset Decision and Preflight

**Status:** Recommended, pending local preflight.

## Candidates compared

### A. Tamaulipas MCQ Exam Image Dataset

- Source: https://data.mendeley.com/datasets/djmynjwjpy/2
- Version listed: 2, published 21 May 2026.
- Reported contents: 5,721 scanned four-option answer sheets.
- Ground truth: a text file with human-observed labels accompanies each image.
- Sheet sizes: 3,669 sheets with 90 items and 2,052 with 100 items.
- License listed on the dataset page: CC BY 4.0.
- Reported challenge: varied marking styles and noise/digitization artifacts.
- Fit: best primary real-data evaluation candidate because it provides scale and item-level labels.
- Risks: two question-count cohorts may not mean two geometrically different layouts, but this must be inspected. Candidate identity-to-roll mapping should not be assumed available. Download size and exact file schema must be confirmed locally.

### B. 145-sheet GitHub dataset

- Source: https://github.com/Fardin-Mahmud/Automated-OMR-Sheet-Evaluation-and-Grading-System
- Reported contents: 145 JPG images, 30 questions, four options, three-column layout and ground-truth CSV.
- Fit: quick smoke-test dataset and easy first template candidate.
- Risks: licensing is not clear from the repository landing page reviewed; do not redistribute until the license is verified. Smaller and less varied.

### C. Wild-OMR

- Source: https://zenodo.org/records/21710005
- Reported contents: 769 released captures of 30 physical sheets, 40 questions and five options; flatbed and phone captures.
- Includes controlled capture variation, ground truth, synthetic generator, baselines and evaluation harness.
- License listed: CC BY 4.0 for data; MIT for code.
- Archive size listed: 1.9 GB.
- Fit: later robustness benchmark.
- Risks: too large and more complex than needed for the first end-to-end implementation.

## Recommended use

1. **Primary real-data evaluation:** Tamaulipas, starting with a small representative subset after confirming file schema and image geometry.
2. **Synthetic deterministic tests:** generate known answer patterns and candidate codes for blank, single, multiple, faint, erased and distorted marks.
3. **Optional quick smoke-test:** the 145-sheet dataset only if its license and layout are verified.
4. **Later robustness suite:** Wild-OMR, not a prerequisite for the MVP.

## Preflight checklist

Before writing a template-specific recognition algorithm:
- [ ] Download only after checking access and storage needs.
- [ ] Record access date, version, source URL and license.
- [ ] Inspect archive contents without modifying the original.
- [ ] Count images and annotation files.
- [ ] Parse a small sample of annotations.
- [ ] Verify answer ordering and number of questions.
- [ ] Visually inspect at least 20 sheets across available cohorts.
- [ ] Measure image dimensions and color modes.
- [ ] Determine whether the template is consistent within each cohort.
- [ ] Identify whether registration markers exist.
- [ ] Determine whether candidate identifiers are present and labelled.
- [ ] Record missing/corrupt files and ambiguous labels.
- [ ] Create a manifest; do not rely on filesystem order.
- [ ] Keep a held-out set for final evaluation.

## Candidate IDs

The Tamaulipas dataset description says sheets were anonymized. Do not assume it provides candidate-to-roll mapping. Implement candidate decoding with synthetic fixtures or a separate controlled template unless inspection proves otherwise.

## Citation links

- Mendeley dataset: https://data.mendeley.com/datasets/djmynjwjpy/2
- Wild-OMR Zenodo record: https://zenodo.org/records/21710005
- 145-sheet repository: https://github.com/Fardin-Mahmud/Automated-OMR-Sheet-Evaluation-and-Grading-System
