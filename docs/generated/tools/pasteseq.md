# pasteseq

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Insert one sequence into another at a deterministic 1-based position

## Document Metadata

- Document ID: `pasteseq-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_transform`
- Legacy names: `pasteseq`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/pasteseq.validation.json`](../validation/pasteseq.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`pasteseq` inserts exactly one sequence record into exactly one main sequence record after a user-supplied position. The EMBOSS-RS v1 implementation is intentionally conservative: it performs a direct insertion only, does not attempt feature-coordinate remapping, and emits one merged sequence record.

## Inputs

The current interface accepts two local sequence inputs, each containing exactly one record, plus a position argument. Position `0` inserts before the start of the main sequence and position `length(main)` inserts at the end. The two records must have the same inferred molecule kind.

## Outputs

The tool emits one merged FASTA sequence record through the shared result path. EMBOSS-RS v1 preserves the identifier and metadata of the main sequence input but drops feature annotations instead of emitting incorrect shifted coordinates.

## Current Status

This method is implemented and exposed through `emboss-rs pasteseq`. Validation currently covers deterministic insertion against committed one-record FASTA fixtures and molecule-mismatch rejection in the Rust tool layer.

## Caveats

The first release supports only single-record inputs and exact in-memory insertion. It does not preserve or remap feature annotations, does not support accession retrieval inputs, and does not attempt overlap-aware merge behavior.

## Declared Artifacts

### Pasteseq main FASTA fixture

- Artifact ID: `pasteseq_main_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/pasteseq_main.fasta`
- Notes: Repository-managed main FASTA fixture used as the insertion target for deterministic pasteseq validation.

### Pasteseq inserted FASTA fixture

- Artifact ID: `pasteseq_insert_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/pasteseq_insert.fasta`
- Notes: Repository-managed one-record FASTA fixture inserted into the main sequence during deterministic pasteseq validation.

## Declared Examples

### Insert one short sequence after position two

- Example ID: `insert_short_sequence_after_position_two`
- Description: Runs `pasteseq` against committed main and inserted FASTA fixtures and emits one merged sequence record after position `2`.
- Referenced artifacts: `pasteseq_main_fasta`, `pasteseq_insert_fasta`
- Parameters:
  - `position` = `2`
- Expected outputs:
  - `pasted_sequence_record`: Merged inserted sequence record (A FASTA sequence record with residues `ACTTGT` and the main-sequence identifier preserved.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `insert_short_sequence_after_position_two`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes

