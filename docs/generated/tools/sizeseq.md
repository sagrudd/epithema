# sizeseq

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Sort sequence records by size in deterministic order

## Document Metadata

- Document ID: `sizeseq-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_transform`
- Legacy names: `sizeseq`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/sizeseq.validation.json`](../validation/sizeseq.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`sizeseq` reorders a sequence stream by record length. The EMBOSS-RS v1 behavior is explicit and deterministic: sort records by descending size and preserve original input order for ties.

## Inputs

The current interface accepts one local sequence input through the shared IO path. Multi-record FASTA is the primary exercised format, while other sequence formats depend on the shared readers already used by the sequence-transform cohort.

## Outputs

The tool emits one reordered FASTA sequence collection. CLI summaries report the source input, the descending-length stable ordering rule, and FASTA output format.

## Current Status

This method is implemented and exposed through `emboss-rs sizeseq`. Validation currently covers deterministic descending size ordering with stable ties against a committed four-record FASTA fixture, and Rust service tests exercise the same repository-managed case.

## Caveats

The first release does not expose ascending sort mode or secondary sort keys beyond stable tie preservation. It is a deterministic record reordering tool, not a reporting-only size summary.

## Declared Artifacts

### Four-record varied-length FASTA fixture

- Artifact ID: `sizeseq_records_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/sizeseq_records.fasta`
- Notes: Repository-managed FASTA fixture containing four records with distinct and tied lengths for deterministic sizeseq validation.

## Declared Examples

### Sort a four-record FASTA fixture by descending size

- Example ID: `sort_records_by_descending_size`
- Description: Runs `sizeseq` against a committed varied-length FASTA fixture and returns the records in descending length order while preserving tie order.
- Referenced artifacts: `sizeseq_records_fasta`
- Expected outputs:
  - `size_sorted_sequence_collection`: Size-sorted sequence collection (A FASTA sequence collection ordered as `long`, `middle`, `short`, then `short_tie`.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `sort_records_by_descending_size`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
