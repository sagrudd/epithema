# runget

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report a normalized public-run manifest for one accession-backed archive run

## Document Metadata

- Document ID: `runget-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `archive_tools`
- Legacy names: `runget`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/runget.validation.json`](../validation/runget.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`runget` is the manifest-oriented public-run companion to `runinfo`. It resolves one provider-qualified archive run accession through the governed archive-provider seam and emits the current normalized public-run manifest table without downloading underlying files.

## Inputs

The current v1 interface accepts exactly one provider-qualified archive run accession, for example `ena:ERR123456`. Local files and inline literals are rejected. The `--download` flag is parsed but explicitly rejected in v1 because direct acquisition is not yet part of the shipped surface.

## Outputs

The result is a stable table report describing the normalized manifest rows for the requested public run. For ENA-backed runs, v1 emits available FASTQ-oriented manifest rows. The method is currently manifest-reporting only and does not materialize files.

## Current Status

This method is implemented and exposed through `emboss-rs runget`. Rust service coverage includes a mocked ENA manifest success path, explicit rejection of `--download`, and an explicit not-supported path for SRA manifest retrieval.

## Caveats

The first release is intentionally narrower than the historical EMBOSS name might suggest. `runget` does not yet download archive payloads, and SRA-backed manifest retrieval is not yet supported. The current curated evidence proves the governed manifest seam and its explicit limits rather than broad provider completeness.

## Declared Artifacts

### Mocked ENA runget case fixture

- Artifact ID: `runget_mocked_ena_manifest_case`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-testkit/tests/fixtures/autodoc/runget_ena_err123456_case.md`
- Notes: Repository-managed case note for the mocked ENA manifest-report path used in Rust service coverage.

## Declared Examples

### Report an ENA public-run manifest without downloading files

- Example ID: `report_ena_run_manifest`
- Description: Resolves a provider-qualified ENA run accession and returns the normalized manifest table through the governed archive reporting path.
- Referenced artifacts: `runget_mocked_ena_manifest_case`
- Expected outputs:
  - `ena_run_manifest_table`: Normalized ENA run manifest table (A stable table report whose rows describe the available ENA FASTQ manifest entries for the requested run.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `report_ena_run_manifest`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes

