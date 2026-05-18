# merger

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Merge two overlapping sequences by longest exact suffix/prefix overlap

## Document Metadata

- Document ID: `merger-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_transform`
- Legacy names: `merger`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/merger.validation.json`](../validation/merger.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`merger` combines exactly one sequence record from a left input and one sequence record from a right input by taking the longest positive exact overlap between the left suffix and right prefix. The EMBOSS-RS v1 implementation is intentionally conservative and emits one merged sequence record rather than attempting assembly graph or consensus behavior.

## Inputs

The current interface accepts two local sequence inputs. Each input must contain exactly one sequence record. The two records must have the same inferred molecule kind. If no positive exact overlap exists, the tool fails instead of concatenating the sequences blindly.

## Outputs

The tool emits one merged FASTA sequence record through the shared result path. CLI summaries report the left and right source inputs, the overlap length used, the exact-overlap merge rule, and FASTA output format.

## Current Status

This method is implemented and exposed through `emboss-rs merger`. Validation currently covers a deterministic one-record overlap merge against committed FASTA fixtures, as well as no-overlap failure behavior in the Rust tool layer.

## Caveats

The first release does not attempt approximate overlap detection, reverse-complement searching, quality-aware merging, or feature-coordinate reconciliation. It uses only exact left-suffix/right-prefix overlap and carries forward left-side metadata without remapping features.

## Declared Artifacts

### Left overlap FASTA fixture

- Artifact ID: `merger_left_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/merger_left.fasta`
- Notes: Repository-managed left input fixture ending with the exact overlap used for deterministic merger validation.

### Right overlap FASTA fixture

- Artifact ID: `merger_right_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/merger_right.fasta`
- Notes: Repository-managed right input fixture beginning with the exact overlap used for deterministic merger validation.

## Declared Examples

### Merge two one-record FASTA inputs by exact overlap

- Example ID: `merge_two_overlapping_records`
- Description: Runs `merger` against committed left and right FASTA fixtures and emits one merged sequence record using the longest exact overlap.
- Referenced artifacts: `merger_left_fasta`, `merger_right_fasta`
- Expected outputs:
  - `merged_sequence_record`: Merged sequence record (A FASTA sequence record with residues `ACGTAAGGG` and an overlap length of `3` reported in the CLI summary.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `merge_two_overlapping_records`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
