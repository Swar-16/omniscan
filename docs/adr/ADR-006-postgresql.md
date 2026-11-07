# ADR-006: PostgreSQL for Future Platform Persistence

- **Status:** Accepted direction; deployment lifecycle open

## Context
The long-term product needs relational integrity for projects, templates, answer keys, jobs, candidates, results and audit history. PostgreSQL is the chosen database.

## Decision
Use PostgreSQL as the canonical structured application database for the platform. The academic CLI remains file-based and does not require a database.

## Consequences
- Use migrations and explicit schema versioning.
- Store metadata and structured records in PostgreSQL.
- Store large image and report artifacts in filesystem/object storage and keep references in the database.
- Local-first deployment needs a deliberate PostgreSQL installation, startup, backup and upgrade strategy. This must be validated before implementation.
- Do not add SQLite solely as a shortcut without revisiting the storage decision explicitly.
