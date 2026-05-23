# complex

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report whole-sequence and sliding-window nucleotide linguistic complexity

## Document Metadata

- Document ID: `complex-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_stats`
- Legacy names: `complex`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/complex.validation.json`](../validation/complex.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`complex` reports nucleotide linguistic complexity over an inclusive k-mer range using the shared EMBOSS-RS complexity core. The first release computes one whole-sequence complexity row per input record and can additionally emit sliding-window rows when `--window` and `--step` are supplied together.

## Inputs

The current tool accepts one local nucleotide input plus required `--k-min` and `--k-max` values. Sliding-window mode additionally requires both `--window` and `--step`. The v1 implementation is strict A/C/G/T only and rejects unsupported or ambiguous symbols instead of approximating them.

## Outputs

The tool emits a stable table report. Whole-sequence rows use `scope=record`, while optional sliding-window rows use `scope=window` with deterministic 1-based inclusive start and end coordinates.

## Current Status

This method is implemented and exposed through `emboss-rs complex`. Validation currently covers deterministic whole-sequence complexity reporting on a committed nucleotide fixture, and Rust tool and service tests also exercise sliding-window output and unsupported-symbol rejection.

## Caveats

The retained exception `complex` remains intentionally narrow in v1. It does not accept ambiguous residues, does not implement richer alphabet models, and does not attempt cross-record comparative summaries beyond the emitted per-record and optional per-window rows.

## Declared Artifacts

### Two-record nucleotide complexity fixture

- Artifact ID: `complex_records_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/complex_records.fasta`
- Notes: Repository-managed FASTA fixture with low- and high-complexity nucleotide examples for deterministic complexity reporting.

## Declared Examples

### Report whole-sequence nucleotide complexity over k=1..2

- Example ID: `report_whole_sequence_complexity`
- Description: Runs `complex` against a committed two-record nucleotide FASTA fixture and emits stable whole-sequence complexity rows.
- Referenced artifacts: `complex_records_fasta`
- Parameters:
  - `--k-min` = `1`
  - `--k-max` = `2`
- Expected outputs:
  - `complexity_table`: Whole-sequence complexity table (A stable report containing one `record` row for the low-complexity fixture sequence and one `record` row for the higher-complexity fixture sequence.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `report_whole_sequence_complexity`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
