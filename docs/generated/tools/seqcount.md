# seqcount

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Count sequence records in one local sequence input

## Document Metadata

- Document ID: `seqcount-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_stream`
- Legacy names: `seqcount`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/seqcount.validation.json`](../validation/seqcount.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`seqcount` counts sequence records through the shared EMBOSS-RS sequence IO layer instead of embedding format-specific counting logic in the tool. The same code path supports the current primary sequence readers for FASTA, FASTQ, EMBL, and GenBank inputs.

## Inputs

The current v1 interface accepts one local sequence input path. Input format is inferred from extension when possible and otherwise from leading file content through the shared sequence-stream detection path.

## Outputs

The tool emits one stable two-column table report with `input` and `count`. CLI output also includes the standard EMBOSS-RS result summary lines describing the input path, record count, and tabular output format.

## Current Status

This method is implemented and exposed through `emboss-rs seqcount`. Validation currently covers single-record and multi-record FASTA inputs plus malformed and empty-input failure behavior.

## Caveats

The first release supports one input path per invocation. Multiple-input aggregation and per-input grouped reporting remain deferred. Empty or malformed inputs fail clearly instead of returning an implicit zero count.

## Declared Artifacts

### Three-record FASTA fixture

- Artifact ID: `three_record_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/three_records.fasta`
- Notes: Repository-managed multi-record FASTA fixture used to validate deterministic counting.

## Declared Examples

### Count records in a three-record FASTA input

- Example ID: `count_three_fasta_records`
- Description: Counts a small multi-record FASTA fixture and returns one stable table row with the source path and record count.
- Referenced artifacts: `three_record_fasta`
- Expected outputs:
  - `sequence_count_report`: Sequence count report (A single-row table report containing the input path and the deterministic record count.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `count_three_fasta_records`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
