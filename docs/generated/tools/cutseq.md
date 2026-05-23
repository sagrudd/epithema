# cutseq

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Split each input sequence record into left and right fragments at one 1-based cut position

## Document Metadata

- Document ID: `cutseq-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_transform`
- Legacy names: `cutseq`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/cutseq.validation.json`](../validation/cutseq.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`cutseq` applies the same interior cut point to every input sequence record and emits the resulting left and right fragments in deterministic order. The v1 implementation is intentionally narrow: it models a single cut position rather than arbitrary interval removal, and it reuses the shared EMBOSS-RS interval and subsequence primitives instead of ad hoc string slicing.

## Inputs

The current tool accepts a local sequence file plus one cut `position`. User-facing coordinates are 1-based and the cut occurs *after* the supplied position, so base or residue `position` belongs to the left fragment. FASTA, FASTQ, EMBL, and GenBank inputs follow the shared sequence IO layer, and the cut position must be interior for every input record.

## Outputs

The tool emits exactly two non-empty fragments per input record, preserving input record order. Fragment identifiers are derived deterministically by suffixing the original accession with `.left` and `.right`. Descriptions, molecule kind, and other record metadata are preserved through the shared `SequenceRecord` model.

## Current Status

This method is implemented and exposed through `emboss-rs cutseq`. Validation currently covers a committed three-record FASTA fixture with a representative interior cut, while Rust unit and service tests cover first-boundary cuts, invalid positions, deterministic naming, and multi-record output ordering.

## Caveats

The v1 scope supports only one cut position and always emits both remaining fragments. Empty fragments are not representable, so positions `0` and `length` are rejected. Interval removal and richer fragment naming policies can be added later through the same governed tool path without changing the documented coordinate convention.

## Declared Artifacts

### Three-record FASTA fixture

- Artifact ID: `three_record_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/three_records.fasta`
- Notes: Repository-managed FASTA fixture used for deterministic cutseq validation.

## Declared Examples

### Cut each record after the second position

- Example ID: `cut_after_second_position`
- Description: Splits each record in a three-record FASTA fixture after position 2 using the shared 1-based cut convention.
- Referenced artifacts: `three_record_fasta`
- Parameters:
  - `position` = `2`
- Expected outputs:
  - `cut_fragments`: Left and right fragments (Each input record produces a `.left` fragment containing positions 1..2 and a `.right` fragment containing the remaining suffix.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `cut_after_second_position`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
