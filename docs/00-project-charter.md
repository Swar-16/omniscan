# OMNISCAN Project Charter

## Purpose

OMNISCAN is a local-first Optical Mark Recognition platform that reads supported answer sheets, identifies candidates, evaluates responses against an answer key and emits auditable results.

## Product strategy

Build a small, correct, measurable academic CLI first. Evolve it into a shared Rust engine used by a desktop app and web app. Optional ML and cloud capabilities are later extensions, not MVP dependencies.

## Product modes

1. Known Template: deterministic processing for a configured sheet.
2. Assisted Template: propose regions and request operator confirmation.
3. Automatic Discovery: future capability for inferring unfamiliar layouts.

Do not promise perfect recognition of every possible form. Page-size independence through normalized geometry is not equivalent to understanding arbitrary layouts.

## Frozen decisions

- Initial deliverable: Rust CLI.
- CV library: OpenCV Rust bindings.
- No ML in academic MVP.
- Public real dataset plus synthetic tests.
- JSON answer key for MVP.
- JSON and CSV output; diagnostic annotated images.
- Positive, negative and partial marking represented in the scoring design, with the MVP implementing only the policies that can be completed and tested.
- Future persistence: PostgreSQL.
- Future interfaces: React + TypeScript web UI and Tauri desktop UI sharing the Rust core.
- Local-first first; cloud deployment later.
- Single user first; multi-user architecture later.
- Modular monolith first; no microservices without evidence.

## Current open decisions

- Final dataset preflight and subset selection.
- Initial sheet template and coordinate calibration.
- Exact supported raster formats for the first vertical slice.
- Exact OpenCV installation/build strategy for the target Windows environment.
- Whether the academic scoring MVP implements partial credit immediately or stages it after basic scoring.

## Definition of academic success

A user can run a documented CLI command on a supported sheet, obtain a candidate identifier where supported, per-question response states, a score, JSON/CSV output and a diagnostic image. Performance and accuracy are measured against disclosed ground truth; no fabricated claims.
