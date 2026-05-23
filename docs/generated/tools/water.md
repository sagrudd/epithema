# water

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Compute deterministic local pairwise alignment between exactly one query and one target record

## Document Metadata

- Document ID: `water-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `pairwise_alignment`
- Legacy names: `water`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/water.validation.json`](../validation/water.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`water` computes one deterministic Smith-Waterman-style local pairwise alignment between exactly one query record and one target record. EMBOSS-RS v1 reports only the highest-scoring local region and renders the primary payload as Stockholm.

## Inputs

The current interface accepts two singleton sequence inputs plus optional gap penalties. Multi-record query or target inputs are rejected clearly. Default scoring mirrors the governed EMBOSS-RS pairwise defaults: nucleotide mode uses match=1 mismatch=-1 gap_open=5 gap_extend=1, while protein mode uses match=2 mismatch=-1 gap_open=8 gap_extend=1.

## Outputs

The result payload is a pairwise local alignment with stable row order `query` then `target`. The CLI renders the payload as Stockholm, and the summary records score, aligned length, identity, the aligned spans on the original query and target sequences, and the effective gap penalties.

## Legacy Context

This first governed `water` contract keeps one historical-style local alignment invocation in view and exercises the EMBOSS-RS v1 local-alignment core against committed fixtures with internal matching regions. It does not claim surface parity with the broader historical EMBOSS parameter set.

## Current Status

This method is implemented and exposed through `emboss-rs water`. Current Rust service coverage exercises committed singleton FASTA fixtures and locks down the highest-scoring local match plus the reported query and target spans.

## Caveats

The first release is intentionally narrow. `water` requires exactly one query and one target record, reports only the single highest-scoring local alignment, and returns a validation error when no positive-scoring local alignment exists.

## Declared Artifacts

### Water query FASTA fixture

- Artifact ID: `water_query_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/water_query.fasta`
- Notes: Repository-managed singleton query fixture containing a local internal nucleotide match.

### Water target FASTA fixture

- Artifact ID: `water_target_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/water_target.fasta`
- Notes: Repository-managed singleton target fixture containing the corresponding highest-scoring local nucleotide region.

## Declared Examples

### Compute one deterministic local nucleotide alignment

- Example ID: `basic_local_alignment`
- Description: Aligns the committed singleton query and target fixtures with the v1 default nucleotide scoring model and returns the highest-scoring local region as Stockholm plus stable query/target span reporting.
- Referenced artifacts: `water_query_fixture`, `water_target_fixture`
- Expected outputs:
  - `alignment_stockholm`: Pairwise Stockholm alignment (A stable Stockholm rendering covering only the best local aligned region.)
  - `alignment_spans`: Local span summary (A stable textual summary reporting the 1-based inclusive query and target spans of the retained local region.)
- Legacy reference: EMBOSS water application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/water.acd`
  - Invocation: `water -asequence water_query.fasta -bsequence water_target.fasta`

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS water application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/water.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `basic_local_alignment`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
