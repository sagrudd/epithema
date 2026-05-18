# megamerger

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Merge two overlapping DNA sequences by longest exact suffix/prefix overlap

## Document Metadata

- Document ID: `megamerger-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_transform`
- Legacy names: `megamerger`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/megamerger.validation.json`](../validation/megamerger.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`megamerger` performs a DNA-only exact overlap merge between one left record and one right record. The first EMBOSS-RS release keeps the method intentionally narrow: it searches only for the longest positive exact overlap between the left suffix and right prefix and emits one merged DNA sequence record.

## Inputs

The current interface accepts two local FASTA-style sequence inputs. Each input must contain exactly one record, and both records must be inferred as DNA. Protein, RNA, multi-record, and provider-backed inputs are rejected in the current v1 service path.

## Outputs

The tool emits one merged DNA FASTA sequence record. CLI summaries report the left and right source inputs, the overlap length used, the exact-overlap merge rule, the DNA-only molecule policy, and FASTA output format.

## Current Status

This method is implemented and exposed through `emboss-rs megamerger`. Validation currently covers a deterministic one-record DNA overlap merge against committed FASTA fixtures, and Rust tool tests also exercise explicit non-DNA rejection.

## Caveats

The first release does not implement approximate overlap finding, ambiguity-tolerant alignment, reverse-complement exploration, or scaffold graph operations. It is a DNA-only exact-overlap join rather than a broader assembly tool.

## Declared Artifacts

### Left DNA overlap FASTA fixture

- Artifact ID: `megamerger_left_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/merger_left.fasta`
- Notes: Repository-managed left DNA input fixture ending with the exact overlap used for deterministic megamerger validation.

### Right DNA overlap FASTA fixture

- Artifact ID: `megamerger_right_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/merger_right.fasta`
- Notes: Repository-managed right DNA input fixture beginning with the exact overlap used for deterministic megamerger validation.

## Declared Examples

### Merge two one-record DNA FASTA inputs by exact overlap

- Example ID: `merge_two_overlapping_dna_records`
- Description: Runs `megamerger` against committed DNA overlap fixtures and emits one merged DNA sequence record.
- Referenced artifacts: `megamerger_left_fasta`, `megamerger_right_fasta`
- Expected outputs:
  - `merged_dna_sequence_record`: Merged DNA sequence record (A DNA FASTA sequence record with residues `ACGTAAGGG` and an overlap length of `3` reported in the CLI summary.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `merge_two_overlapping_dna_records`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
