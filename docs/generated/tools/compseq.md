# compseq

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report deterministic residue composition counts and frequencies for sequence records

## Document Metadata

- Document ID: `compseq-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_stats`
- Legacy names: `compseq`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/compseq.validation.json`](../validation/compseq.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`compseq` reports residue composition for each input record plus one aggregate summary across all records. The EMBOSS-RS v1 surface intentionally keeps the report deterministic and simple: count normalized non-gap symbols, preserve ambiguity or stop symbols as observed, and compute frequencies over the non-gap denominator only.

## Inputs

The current v1 interface accepts one local sequence input path. Inputs are loaded through the shared EMBOSS-RS readers for FASTA, FASTQ, EMBL, and GenBank, and may contain nucleotide, protein, or mixed record sets.

## Outputs

The tool emits a stable table report with columns `scope`, `record`, `molecule`, `length`, `residue`, `count`, and `frequency`. `scope` is either `record` or `aggregate`. Aggregate rows use `record=ALL` and `molecule=mixed` in the first release.

## Statistics Model

Residues are normalized case-insensitively before counting. Gap symbols `-` are ignored and excluded from the frequency denominator. All other normalized symbols, including ambiguity codes such as `N` and stop symbols such as `*`, are counted exactly as observed. Frequencies are reported as fractions over all non-gap symbols in the corresponding record or aggregate set.

## Current Status

This method is implemented and exposed through `emboss-rs compseq`. Validation currently covers nucleotide composition, protein composition, ambiguity and stop-symbol handling, per-record plus aggregate reporting, and empty-input failure.

## Caveats

The first release does not infer richer chemistry-aware residue classes or reject mixed record sets. Composition is purely symbol-based after shared sequence normalization.

## Declared Artifacts

### Nucleotide composition fixture

- Artifact ID: `nucleotide_pattern_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/nucleotide_pattern_records.fasta`
- Notes: Repository-managed nucleotide fixture used for deterministic composition validation.

## Declared Examples

### Compute per-record and aggregate composition

- Example ID: `per_record_and_aggregate_composition`
- Description: Reports normalized residue counts and frequencies for each input record plus one aggregate summary across the whole input set.
- Referenced artifacts: `nucleotide_pattern_fixture`
- Expected outputs:
  - `composition_table`: Residue composition table (A stable tabular report containing per-record and aggregate residue counts and non-gap frequencies.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `per_record_and_aggregate_composition`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes

