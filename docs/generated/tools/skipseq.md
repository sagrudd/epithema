# skipseq

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Skip the first N sequence records and return the remainder

## Document Metadata

- Document ID: `skipseq-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_stream`
- Legacy names: `skipseq`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/skipseq.validation.json`](../validation/skipseq.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`skipseq` removes a non-negative number of leading sequence records from a local input stream and returns the remaining records in stable source order. The tool reuses the shared Epithema sequence loader and sequence-collection result path rather than embedding format-specific stream handling.

## Inputs

The current v1 interface accepts one local sequence input path plus one non-negative skip count. The count is interpreted as the number of leading records to discard before emitting the remainder.

## Outputs

The tool emits the remaining sequence records through the shared FASTA output path. CLI output also includes the standard Epithema method summary lines reporting the input path, effective skipped count, total input count, returned record count, and FASTA output format.

## Current Status

This method is implemented and exposed through `epithema skipseq`. Validation currently covers zero-skip behavior, single-record skipping, interior skip counts, skip-all, skip-beyond-end, and malformed or empty-input failure behavior.

## Caveats

The first release supports one input path and one skip count per invocation. Negative skip counts are rejected by argument parsing. Skip counts greater than the number of records are clamped to the input length and return an empty output stream rather than failing.

## Declared Artifacts

### Three-record FASTA fixture

- Artifact ID: `three_record_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/three_records.fasta`
- Notes: Repository-managed multi-record FASTA fixture used to validate stable skip behavior.

## Declared Examples

### Skip the first record from a three-record FASTA input

- Example ID: `skip_first_record`
- Description: Drops the first record from a small FASTA fixture and returns the remaining suffix in original order.
- Referenced artifacts: `three_record_fasta`
- Parameters:
  - `count` = `1`
- Expected outputs:
  - `remaining_sequence_collection`: Remaining sequence collection (A FASTA sequence collection containing the second and third records from the source fixture.)

## Provenance

- Curated by: epithema maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `skip_first_record`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
