# ADR-010: Version Templates and Processing Configurations

- **Status:** Accepted

## Context
A result is only reproducible if the template and processing parameters used to generate it are known.

## Decision
Version templates, answer keys and processing configurations. Record the applicable version or checksum with each processing run.

## Consequences
- Configuration changes do not silently alter historical results.
- Benchmark results can be tied to exact configurations.
- The platform needs migration and compatibility policies for old configurations.
