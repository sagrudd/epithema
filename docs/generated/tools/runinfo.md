# runinfo

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Normalize ENA or SRA archive metadata for one accession-backed archive object

## Document Metadata

- Document ID: `runinfo-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `archive_tools`
- Legacy names: `runinfo`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/runinfo.validation.json`](../validation/runinfo.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`runinfo` normalizes archive metadata for one accession-backed public run object. In v1 it resolves one provider-qualified ENA or SRA accession through the governed archive-provider seam and emits a stable table-oriented metadata summary.

## Inputs

The current interface accepts exactly one provider-qualified archive accession such as `ena:ERR123456` or `sra:SRR123456`. Local file inputs and inline literals are rejected deliberately because `runinfo` is an archive lookup tool, not a file parser.

## Outputs

The result is a stable table report plus provider-summary lines. ENA-backed runs currently normalize into table rows that distinguish available artifact classes such as FASTQ payloads. SRA-backed lookups currently return provider-tagged summary output with narrower normalized table coverage.

## Current Status

This method is implemented and exposed through `epithema runinfo`. Rust service coverage includes mocked ENA run metadata normalization and mocked SRA run metadata lookup. Those tests prove the current provider seams without claiming that harvested live-provider evidence has been checked in yet.

## Caveats

The v1 surface is intentionally conservative. `runinfo` does not accept local files, free-text metadata blobs, or download requests. ENA normalization is currently richer than SRA normalization, so provider-specific result breadth is not yet symmetrical.

## Declared Artifacts

### Mocked ENA runinfo case fixture

- Artifact ID: `runinfo_mocked_ena_case`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-testkit/tests/fixtures/autodoc/runinfo_ena_err123456_case.md`
- Notes: Repository-managed case note for the mocked ENA metadata normalization path used in Rust service coverage.

## Declared Examples

### Normalize ENA public run metadata into a stable table report

- Example ID: `normalize_ena_run_metadata`
- Description: Resolves a provider-qualified ENA run accession and returns the current governed metadata table and provider summary.
- Referenced artifacts: `runinfo_mocked_ena_case`
- Expected outputs:
  - `ena_run_metadata_table`: Normalized ENA run metadata table (A stable table report whose first rows describe the available ENA FASTQ artifact class and associated manifest fields.)

## Provenance

- Curated by: epithema maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `normalize_ena_run_metadata`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
