# descseq

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report stable sequence-record descriptions and metadata summaries in tabular form

## Document Metadata

- Document ID: `descseq-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_edit`
- Legacy names: `descseq`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/descseq.validation.json`](../validation/descseq.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`descseq` reports stable per-record sequence descriptions and metadata using the shared Epithema sequence and annotated-record models. It is useful for plain FASTA inputs and richer EMBL or GenBank inputs because it summarizes the typed metadata already carried by the core record representation instead of reparsing format-specific text ad hoc.

## Inputs

The current v1 tool accepts one local sequence input path and preserves source record order. Plain sequence records and annotated records are both supported through the shared IO layer.

## Outputs

The output is a stable tabular report with one row per input record. The current schema reports `ordinal`, `identifier`, `display_name`, `description`, `length`, `molecule`, `alphabet`, `feature_count`, `source`, `organism`, and `topology`. Missing metadata values are rendered as `-` in the CLI table output.

## Current Status

This method is implemented and exposed through `epithema descseq`. Validation currently covers both plain multi-record FASTA input and annotated GenBank input so the reported schema is locked down for both unannotated and annotation-aware records.

## Caveats

The v1 output is intentionally conservative and metadata-driven. It reports only fields already represented cleanly in the shared core types and does not attempt format-specific free-text harvesting beyond those typed fields.

## Declared Artifacts

### Three-record FASTA fixture

- Artifact ID: `three_record_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/three_records.fasta`
- Notes: Repository-managed plain FASTA fixture used to validate source-order summary reporting.

### Annotated GenBank fixture

- Artifact ID: `annotated_feature_genbank`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/annotated_feature.gbk`
- Notes: Repository-managed annotated GenBank fixture used to validate annotation-aware summary reporting.

## Declared Examples

### Summarize plain FASTA records in stable source order

- Example ID: `summarize_plain_fasta_records`
- Description: Reports one table row per FASTA record with identifier, description, length, molecule, and alphabet fields.
- Referenced artifacts: `three_record_fasta`
- Expected outputs:
  - `sequence_description_rows`: Sequence description rows (The plain sequence input is summarized as a stable per-record tabular report.)

## Provenance

- Curated by: epithema maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `summarize_plain_fasta_records`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
