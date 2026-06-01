# syco

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report a bounded synonymous codon preference profile and emit a line-plot contract

## Document Metadata

- Document ID: `syco-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `nucleotide_plots`
- Legacy names: `syco`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/syco.validation.json`](../validation/syco.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`syco` is the final bounded plotting continuation candidate after the shipped `banana` slice. The EMBOSS-RS v1 surface computes one deterministic coding-sequence synonymous codon preference profile for exactly one nucleotide record against a reference codon-usage source and emits both a stable analytical table and a typed single-series line-plot contract.

## Inputs

This tool accepts exactly one coding nucleotide sequence record plus one reference codon-usage source. v1 requires the sequence length to be divisible by three, rejects ambiguous or internally stopped codons, accepts either a coding-sequence file or a normalized codon profile TSV as the reference source, and treats `--codon-window` and `--codon-step` as positive codon counts.

## Outputs

The implementation emits a stable analytical table with `sequence_id`, `window_start`, `window_end`, `window_length`, `codon_window_length`, `sense_codon_count`, and `syco_score`. The staged v1 line-plot contract remains single-series and plots the analytically derived `syco_score` values from the same governed computation path.

## Plotting Integration

Rust does not render figures. The formal contract emitted by `syco` is the governed handoff to the sister `emboss-r` package, which owns graphical rendering. This shipped `syco` slice intentionally stays single-series and renderer-agnostic even though the analytical surface is driven by coding-sequence and codon-usage inputs.

## Current Status

This method is implemented and exposed through `emboss-rs syco`. At this task boundary the governed surface ships with executable validation and a generated validation stub, but canonical checked-in compared evidence for the analytical table and line-plot contract has not landed yet.

## Caveats

v1 supports only the single-record bounded `syco` profile against one supplied reference codon-usage source and does not add Rust-side rendering, multi-series plotting, codon-usage panelization, or a generalized plotting framework. Additional continuation should remain bounded by the explicit `syco` seam stop conditions already recorded in governance.

## Declared Artifacts

### Coding nucleotide fixture for governed syco validation

- Artifact ID: `syco_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/syco_coding_nucleotide.fasta`
- Notes: Repository-managed coding nucleotide fixture used for deterministic syco validation.

### Reference codon profile source fixture for governed syco validation

- Artifact ID: `syco_reference_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/codon_reference.fasta`
- Notes: Repository-managed coding reference fixture used to derive the bounded syco reference codon-usage profile.

## Declared Examples

### Compute a deterministic bounded syco profile

- Example ID: `syco_profile_example`
- Description: Reports deterministic coding-window synonymous codon preference rows from the committed coding nucleotide fixture against the committed codon reference fixture and emits a governed single-series syco-score line-plot contract from the same analytical run.
- Referenced artifacts: `syco_fixture`, `syco_reference_fixture`
- Parameters:
  - `codon_window` = `2`
  - `codon_step` = `1`
- Expected outputs:
  - `syco_table`: Syco analytical table (Stable coding-window syco rows with explicit synonymous preference scores.)
  - `syco_plot`: Syco-score line-plot contract (The governed single-series syco-score line-plot contract JSON emitted from the same deterministic analytical run.)
- Legacy reference: EMBOSS syco application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/syco.acd`
  - Invocation: `syco -seqall syco_coding_nucleotide.fasta -cfile codon_reference.fasta -graph data`

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS syco application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/syco.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `syco_profile_example`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes
