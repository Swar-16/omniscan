# ADR-005: Share One Processing Core Across Interfaces

- **Status:** Accepted

## Context
The roadmap includes CLI, desktop and web experiences. Duplicating OMR logic would create inconsistent results and multiply maintenance.

## Decision
Keep recognition, candidate resolution and evaluation in a shared Rust core. Interfaces call into that core through suitable application boundaries.

## Consequences
- Consistent behavior across CLI and GUI.
- Core APIs must not depend on a specific UI.
- Web deployment may require a service/API boundary; desktop may call a local library or local service.
