# pepstats

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report deterministic protein summary statistics for sequence records

## Document Metadata

- Document ID: `pepstats-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_stats`
- Legacy names: `pepstats`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/pepstats.validation.json`](../validation/pepstats.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`pepstats` reports a conservative first-release set of protein statistics for each input record. The EMBOSS-RS v1 implementation includes raw sequence length, residue length excluding stop symbols, stop-count, amino-acid composition counts and frequencies, and a deterministic average-residue molecular-weight estimate.

## Inputs

The current interface accepts one local protein input path. Inputs are loaded through the shared EMBOSS-RS readers for FASTA, FASTQ, EMBL, and GenBank. Nucleotide inputs are rejected.

## Outputs

The tool emits a stable table report with columns `section`, `record`, `metric_or_residue`, `value_or_count`, `frequency`, and `notes`. `section=summary` rows report scalar metrics such as sequence length and molecular weight. `section=composition` rows report residue counts and frequencies.

## Metric Model

Input residues are normalized case-insensitively. Gap symbols are excluded from composition counts. Stop symbols `*` are counted in composition but excluded from `residue_length` and from molecular-weight estimation. Molecular weight uses the shared EMBOSS-RS average residue masses with one water molecule added once per chain.

## Legacy Context

This acceptance anchor keeps one historical-style `pepstats` reporting example in view and compares the EMBOSS-RS tabular payload against a committed expected output. The comparison validates the governed first-release metric set rather than the broader historical EMBOSS biochemical-report surface.

## Current Status

This method is implemented and exposed through `emboss-rs pepstats`. Validation currently covers protein summary rows, composition frequencies, stop-symbol handling, unsupported ambiguous-residue failure for mass estimation, and nucleotide-input rejection.

## Caveats

The first release does not estimate isoelectric point, extinction coefficient, or advanced residue-class summaries. Ambiguous protein symbols that are not supported by the shared molecular-weight helper, such as `X` or `Z`, cause the run to fail instead of being approximated.

## Declared Artifacts

### Protein statistics fixture

- Artifact ID: `protein_stats_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/protein_stats_records.fasta`
- Notes: Repository-managed protein fixture used for deterministic pepstats validation.

## Declared Examples

### Compute per-record protein statistics

- Example ID: `protein_summary_statistics`
- Description: Reports scalar summary metrics and per-residue composition rows for a small protein fixture.
- Referenced artifacts: `protein_stats_fixture`
- Expected outputs:
  - `pepstats_report`: Protein statistics table (A stable table containing per-record summary metrics and amino-acid composition frequencies.)
- Legacy reference: EMBOSS pepstats application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/pepstats.acd`
  - Invocation: `pepstats -sequence protein_stats_records.fasta -stdout yes`

## Provenance

- Curated by: OpenAI Codex
- Source references:
  - EMBOSS pepstats application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/pepstats.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `protein_summary_statistics`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes

