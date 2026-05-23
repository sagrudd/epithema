# distmat

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Pairwise p-distance matrix for equal-length sequence sets

## Document Metadata

- Document ID: `distmat-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `alignment_analysis`
- Legacy names: `distmat`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/distmat.validation.json`](../validation/distmat.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`distmat` computes a deterministic p-distance matrix for one local sequence set. The current EMBOSS-RS implementation uses the shared sequence loader and reports mismatch fraction as `mismatches / sequence_length` for each pair in stable input order.

## Inputs

The current v1 interface accepts exactly one local sequence-set input path. All records must have equal length before the distance matrix can be derived.

## Outputs

CLI and service output is a stable square table with one row per input record and one column per input record. Diagonal entries are zero, and off-diagonal entries report pairwise mismatch fraction as fixed-precision decimal text.

## Current Status

This method is implemented and exposed through `emboss-rs distmat`. Current Rust service coverage exercises the committed equal-length three-record FASTA fixture and locks down the expected p-distance rows in stable source order.

## Caveats

Mixed-length inputs fail validation instead of being padded, aligned, or compared over partial overlap. The first release reports simple p-distance only; it does not yet expose alternate distance models.

## Declared Artifacts

### Equal-length FASTA fixture

- Artifact ID: `equal_length_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/three_records.fasta`
- Notes: Repository-managed three-record FASTA fixture with equal-length sequences used for deterministic p-distance validation.

## Declared Examples

### Compute a p-distance matrix for three equal-length FASTA records

- Example ID: `compute_p_distance_matrix_for_equal_length_records`
- Description: Loads the committed three-record FASTA fixture and reports the pairwise mismatch fractions in stable input order.
- Referenced artifacts: `equal_length_fasta`
- Expected outputs:
  - `distance_matrix_table`: P-distance matrix table (A square table with zero diagonal entries and fixed-precision mismatch fractions for each record pair.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `compute_p_distance_matrix_for_equal_length_records`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
