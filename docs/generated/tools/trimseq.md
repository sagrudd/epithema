# trimseq

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Trim explicit residue counts from the left and right ends of sequence records

## Document Metadata

- Document ID: `trimseq-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_edit`
- Legacy names: `trimseq`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/trimseq.validation.json`](../validation/trimseq.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`trimseq` removes an explicit number of residues from the left and right ends of each input record. The EMBOSS-RS v1 implementation keeps the model narrow and deterministic: trimming is by residue count, applied uniformly to every record, and rejected if the requested total would exhaust a sequence.

## Inputs

The current tool accepts one local sequence input plus optional `--left` and `--right` counts. Counts are non-negative and default to zero. Multi-record inputs are supported, and the same trim counts are applied to every record in input order.

## Outputs

Output is a normalized sequence collection rendered through the shared result and CLI layers. Trimmed records retain their original identifiers, descriptions, and molecule metadata when the resulting sequence remains valid.

## Current Status

This method is implemented and exposed through `emboss-rs trimseq`. Validation currently covers representative left-and-right trimming against a committed FASTA fixture, while Rust service tests cover deterministic output ordering and exhaustion rejection.

## Caveats

The v1 scope trims only fixed residue counts and does not implement motif-based clipping, adapter discovery, or quality-aware trimming. If `--left` plus `--right` would remove all residues from a record, the tool fails rather than emitting an empty sequence.

## Declared Artifacts

### Three-record FASTA fixture

- Artifact ID: `three_record_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/three_records.fasta`
- Notes: Repository-managed FASTA fixture used for deterministic trimseq validation.

## Declared Examples

### Trim one residue from the left and right ends of each record

- Example ID: `trim_one_residue_from_each_end`
- Description: Runs `trimseq` against the committed three-record FASTA fixture with one residue removed from each end of every input record.
- Referenced artifacts: `three_record_fasta`
- Parameters:
  - `--left` = `1`
  - `--right` = `1`
- Expected outputs:
  - `trimmed_sequences`: Trimmed sequence output (Stable FASTA output in which `alpha` becomes `CG`, `beta` becomes `TT`, and `gamma` becomes `GC`.)
- Legacy reference: EMBOSS trimseq application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/trimseq.acd`
  - Invocation: `trimseq -sequence three_records.fasta -left 1 -right 1 -outseq stdout`

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `trim_one_residue_from_each_end`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
