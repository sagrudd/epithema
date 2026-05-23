# degapseq

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Remove '-' and '.' gap characters from sequence records

## Document Metadata

- Document ID: `degapseq-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_edit`
- Legacy names: `degapseq`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/degapseq.validation.json`](../validation/degapseq.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`degapseq` removes gap markers from sequence records and emits the normalized residue stream through the shared EMBOSS-RS sequence model. The v1 implementation is intentionally literal: only `-` and `.` are stripped, record ordering is preserved, and no alignment-aware coordinate bookkeeping is attempted.

## Inputs

The current tool accepts local sequence inputs through the governed sequence IO path. Multi-record inputs are supported. FASTA is the primary exercised format in the current validation path, while other sequence formats depend on the shared IO readers.

## Outputs

Output is a normalized sequence collection rendered through the shared result and CLI layers. Identifiers, descriptions, and preserved record metadata remain attached to each transformed sequence record.

## Current Status

This method is implemented and exposed through `emboss-rs degapseq`. Validation currently covers deterministic gap removal against a committed two-record FASTA fixture, and the Rust service path is exercised against the same repository-managed input.

## Caveats

The first release strips only the explicit gap characters `-` and `.`. It does not preserve alignment column structure or emit an annotated alignment result, so it should be treated as a sequence-normalization step rather than as alignment editing.

## Declared Artifacts

### Two-record gapped FASTA fixture

- Artifact ID: `gapped_records_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/gapped_records.fasta`
- Notes: Repository-managed FASTA fixture containing explicit `-` and `.` gap markers for deterministic degapseq validation.

## Declared Examples

### Remove explicit gap markers from a committed FASTA fixture

- Example ID: `remove_gap_markers`
- Description: Runs `degapseq` against a repository-managed two-record FASTA fixture and removes `-` and `.` while preserving record order.
- Referenced artifacts: `gapped_records_fasta`
- Expected outputs:
  - `degapped_sequences`: Gap-free sequence output (Stable FASTA output with `ACGT` and `TTA` as the normalized residues for the two fixture records.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `remove_gap_markers`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
