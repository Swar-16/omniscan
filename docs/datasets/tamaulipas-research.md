# Tamaulipas MCQ Exam Image Dataset — Research & Profile

## 1. Overview & Provenance

The primary real-world assessment dataset selected for OMNISCAN evaluation is the **Tamaulipas Multiple-Choice-Question Exam Image Dataset for Optical Mark Recognition Research**.

- **Repository:** Mendeley Data
- **DOI / Link:** [10.17632/djmynjwjpy.2](https://data.mendeley.com/datasets/djmynjwjpy/2) (Version 2)
- **Associated Journal Publication:**
  - *Title:* Unsupervised Optical Mark Recognition on Answer Sheets for Massive Printed Multiple-Choice Tests
  - *Authors:* Yahir Hernández-Mier, Marco Aurelio Nuño-Maganda, Said Polanco-Martagón, Guadalupe Acosta-Villarreal, and Rubén Posada-Gómez
  - *Journal:* *Journal of Imaging* (MDPI), 2025
- **License:** **Creative Commons Attribution 4.0 International (CC BY 4.0)**
  - Permits sharing and adaptation for academic and commercial purposes, provided appropriate attribution and citation are given.

---

## 2. Dataset Reconciliation: Sheet Counts & Volume

Documentation across published sources reports two slightly different totals:
1. **Mendeley Data Repository (Version 2):** Reports **5,721** scanned answer sheets:
   - **Cohort A (90 questions):** 3,669 sheets
   - **Cohort B (100 questions):** 2,052 sheets
2. **MDPI Journal Publication:** Mentions **6,029** total physical answer sheets collected across high school and university admission exams in Tamaulipas, Mexico, representing 564,040 four-option responses.

**Reconciliation Note:**  
The difference of 308 sheets corresponds to physical examination papers that were withheld or removed during anonymization, quality filtering, or digitization review prior to Mendeley publication. OMNISCAN's evaluation targets the 5,721 officially released sheets in Mendeley Version 2.

---

## 3. Examination Cohorts & Layout Structure

The dataset contains two distinct examination cohorts:

| Cohort | Question Count | Options per Question | Total Published Sheets | Column Layout |
|---|---|---|---|---|
| **Cohort 90** | 90 questions | 4 options (`A`, `B`, `C`, `D`) | 3,669 | Multi-column grid |
| **Cohort 100** | 100 questions | 4 options (`A`, `B`, `C`, `D`) | 2,052 | Multi-column grid |

### Physical Sheet Attributes
- **Registration Features:** Black perimeter reference squares (fiducials) designed for homography or affine coordinate transformation.
- **Marking Implements:** Pencil and ink marks made by students under real testing conditions.
- **Real-World Variations:** Varying fill darkness, faint marks, partial erasures, smudges, and slight skew.

---

## 4. Ground-Truth Labels & Anonymization

1. **Format:** Plain text files (`.txt`) accompanying each image scan.
2. **Granularity:** Question-level human observations recording student responses (`A`, `B`, `C`, `D`, blanks, or multiple marks).
3. **Candidate Identification Limitation:**
   - In accordance with privacy and institutional ethics policies, student personal information and roll numbers were anonymized or masked on the physical sheets.
   - **Critical Architectural Decision:** OMNISCAN does not assume candidate-to-roll mapping exists in the public ground-truth labels. In accordance with ADR-007, candidate ID bubble decoding is benchmarked using synthetic fixtures (`configs/templates/synthetic-standard-30q-v1.json`).

---

## 5. Staging & Calibration Roadmap

In accordance with OMNISCAN's core architectural principle (*"Never invent coordinates or pretend an uninspected sheet has been calibrated"*):

- **Milestone M1:** Defines the normalized JSON template schema and validates a synthetic reference template (`synthetic-standard-30q-v1.json`).
- **Milestone M2:** Conducts image ingestion on representative Tamaulipas sample sheets, measures raster DPI and bounding geometry, and calibrates the real `tamaulipas-cohort-90-v1.json` template.
- **Large Dataset Policy:** The 5,721 image files will **not** be committed to Git. Scans remain in local external storage (e.g., `data/external/tamaulipas/`) referenced via JSON manifests.

