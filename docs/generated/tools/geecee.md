# geecee

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report deterministic GC counts and GC percentages for nucleotide sequence records

## Document Metadata

- Document ID: `geecee-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_stats`
- Legacy names: `geecee`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/geecee.validation.json`](../validation/geecee.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`geecee` reports GC statistics for each input record plus one aggregate summary across all records. The EMBOSS-RS v1 implementation keeps the model deliberately conservative: only canonical A/C/G/T/U symbols contribute to the GC denominator, while ambiguous non-gap symbols are counted separately and excluded from the percentage.

## Inputs

The current interface accepts one local nucleotide input path. Inputs are loaded through the shared EMBOSS-RS readers for FASTA, FASTQ, EMBL, and GenBank. DNA and RNA records are supported; non-nucleotide records are rejected.

## Outputs

The tool emits a stable table report with columns `scope`, `record`, `length`, `gc_count`, `gc_denominator`, `ambiguous_count`, and `gc_percent`. `scope` is either `record` or `aggregate`. Aggregate rows use `record=ALL`.

## GC Model

Input residues are normalized case-insensitively before counting. Gap symbols `-` are ignored completely. Canonical `G` and `C` contribute to the numerator and denominator. Canonical `A`, `T`, and `U` contribute to the denominator only. Ambiguous or otherwise non-canonical non-gap symbols such as `N` are counted in `ambiguous_count` and excluded from the GC percentage.

## Current Status

This method is implemented and exposed through `emboss-rs geecee`. Validation currently covers DNA input, RNA input, ambiguity exclusion from the denominator, aggregate reporting, and non-nucleotide failure.

## Caveats

The first release does not attempt probabilistic treatment of ambiguous nucleotide codes. Ambiguous symbols are tracked separately rather than fractionally contributing to GC. Mixed inputs containing non-nucleotide records are rejected. Empty FASTA records are also rejected by the shared EMBOSS-RS sequence parser before GC calculation begins.

## Declared Artifacts

### Nucleotide GC fixture

- Artifact ID: `nucleotide_pattern_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/nucleotide_pattern_records.fasta`
- Notes: Repository-managed nucleotide fixture used for deterministic GC reporting validation.

## Declared Examples

### Per-record and aggregate GC reporting

- Example ID: `per_record_and_aggregate_gc`
- Description: Reports GC counts, canonical-symbol denominators, ambiguity counts, and GC percentage for each input record plus one aggregate summary row.
- Referenced artifacts: `nucleotide_pattern_fixture`
- Expected outputs:
  - `gc_table`: GC statistics table (A stable tabular report containing per-record and aggregate GC statistics over canonical nucleotide symbols.)

## Provenance

- Curated by: OpenAI Codex
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `per_record_and_aggregate_gc`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
