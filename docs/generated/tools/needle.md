# needle

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Compute deterministic global pairwise alignment between exactly one query and one target record

## Document Metadata

- Document ID: `needle-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `pairwise_alignment`
- Legacy names: `needle`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/needle.validation.json`](../validation/needle.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`needle` computes a deterministic global pairwise alignment between exactly one query record and one target record. The EMBOSS-RS v1 implementation uses the shared global-alignment core and renders the primary payload as Stockholm.

## Inputs

The current interface accepts two singleton sequence inputs plus optional gap penalties. Multi-record query or target inputs are rejected clearly. Default scoring differs by molecule class: nucleotide mode uses match=1 mismatch=-1 gap_open=5 gap_extend=1, while protein mode uses match=2 mismatch=-1 gap_open=8 gap_extend=1.

## Outputs

The result payload is a pairwise alignment with stable row order `query` then `target`. The CLI renders the payload as Stockholm, and the summary records score, aligned length, identity, and the effective gap penalties.

## Legacy Context

This acceptance anchor keeps one simple historical-style `needle` invocation in view and compares the EMBOSS-RS alignment payload against a committed expected Stockholm rendering. The comparison is intentionally narrow: it validates the governed v1 scoring and rendering path for the committed fixture pair rather than claiming full parameter-surface parity with historical EMBOSS.

## Current Status

This method is implemented and exposed through `emboss-rs needle`. Validation covers execution against committed singleton fixtures, rejection of multi-record query input, and one compared acceptance-anchor case against a committed expected Stockholm output.

## Caveats

The first release is intentionally narrow. `needle` requires exactly one query and one target record, and the compared acceptance case exercises the default nucleotide path only.

## Declared Artifacts

### Needle query FASTA fixture

- Artifact ID: `needle_query_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/needle_query.fasta`
- Notes: Repository-managed singleton query fixture for the governed needle acceptance case.

### Needle target FASTA fixture

- Artifact ID: `needle_target_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/needle_target.fasta`
- Notes: Repository-managed singleton target fixture for the governed needle acceptance case.

## Declared Examples

### Compute one deterministic global nucleotide alignment

- Example ID: `basic_alignment`
- Description: Aligns the committed singleton query and target fixtures with the v1 default nucleotide scoring model and renders the primary payload as Stockholm.
- Referenced artifacts: `needle_query_fixture`, `needle_target_fixture`
- Expected outputs:
  - `alignment_stockholm`: Pairwise Stockholm alignment (A stable Stockholm rendering with rows `query` and `target` and one gap in the target row.)
- Legacy reference: EMBOSS needle application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/needle.acd`
  - Invocation: `needle -asequence needle_query.fasta -bsequence needle_target.fasta -gapopen 10 -gapextend 0.5`

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS needle application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/needle.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `basic_alignment`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes
