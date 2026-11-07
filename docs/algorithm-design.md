# Algorithm Design

## 1. Pipeline overview

```text
Input scan
   |
Decode + validate
   |
Quality checks
   |
Grayscale / denoise / threshold
   |
Registration detection + geometric alignment
   |
Template coordinate transform
   |
ROI extraction
   |
Bubble fill measurement
   |
Response-state classification
   |
Candidate identifier decoding + mapping
   |
Answer-key evaluation
   |
JSON / CSV + annotated diagnostics
```

## 2. Image preprocessing

Investigate grayscale conversion, denoising and thresholding. Compare Otsu global thresholding with adaptive thresholding on the selected dataset. Do not assume one method is universally superior; record results by scan condition.

Keep the original image unchanged. Every derived image should be a separate processing artifact.

## 3. Geometric alignment

If the template has registration markers or stable page features, detect them and estimate a geometric transform. Use an affine transform for rotation/scale/translation when adequate; use a homography when perspective distortion requires a projective mapping and enough reliable correspondences are available.

Reject or flag poor alignment rather than extracting ROIs from a badly registered page.

## 4. Template and normalized coordinates

Represent regions relative to a reference page coordinate system. A template should define:
- reference width and height;
- page orientation;
- registration features;
- candidate identifier region;
- question and option regions;
- question type and option count;
- any calibration or tolerance parameters.

Normalized coordinates help handle different raster resolutions. They do not automatically solve unknown layout recognition.

## 5. Bubble fill measurement

For each expected bubble ROI, compute a darkness or foreground-density measure. One simple metric is:

\[
d = \frac{\text{foreground pixels inside bubble mask}}
         {\text{total pixels inside bubble mask}}
\]

A circular mask is preferable when bubble geometry is known. A rectangular ROI may be used for early experiments, but it can include border and neighboring artifacts.

An integral image can accelerate rectangular sum queries: build it in \(O(W H)\), then query each axis-aligned rectangle in \(O(1)\). For circular regions, use precomputed masks or another measured approach.

## 6. Response classification

Use calibrated thresholds and explicit margins between the strongest candidate and other candidates. A question can be:
- `Selected(option)`;
- `Blank`;
- `Multiple(options)`;
- `Ambiguous(reason)`.

Do not force a low-confidence bubble into a selected state. Thresholds must be calibrated on training/validation samples and evaluated on held-out scans.

## 7. Candidate identification

Decode the identifier using the method supported by the chosen template. For bubble-coded digits, evaluate each digit column/row using the same fill analysis, with explicit handling for blank or multiple marks. Resolve the decoded code through a mapping table.

Use a hash map for candidate-code lookup: expected \(O(1)\) lookup. Detect duplicate codes before processing.

## 8. Evaluation

For each question, compare the recognized response with the answer key and apply the configured marking policy. Define behavior for:
- correct response;
- incorrect response;
- blank response;
- multiple marks;
- ambiguous response;
- multiple-select response;
- partial credit.

The evaluation module should be independent of image processing and tested using synthetic domain objects.

## 9. Data structures and complexity

| Need | Candidate structure | Rationale |
|---|---|---|
| Candidate code lookup | `HashMap` | Expected constant-time lookup |
| Ordered question responses | `Vec` | Contiguous storage and indexed access |
| Duplicate detection | `HashSet` | Expected constant-time membership |
| Irregular spatial lookup | Spatial grid or R-tree | Add only if geometry/query volume justifies it |
| Batch scheduling | Bounded queue | Controls peak memory |
| Per-question scoring | Sequential/vector iteration | \(O(Q)\) per sheet |

Avoid introducing complex data structures merely to make the project appear sophisticated. Measure actual bottlenecks first.

## 10. Duplicate scans

Use cryptographic hashes to identify byte-identical files. Perceptual hashes can flag visually similar scans, but must not automatically discard them: different candidates can have near-identical blank forms. Treat similarity as a review signal.
