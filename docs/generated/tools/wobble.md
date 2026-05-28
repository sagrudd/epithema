# wobble

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report a bounded third-base-position variability profile and emit a line-plot contract

## Document Metadata

- Document ID: `wobble-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `nucleotide_plots`
- Legacy names: `wobble`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/wobble.validation.json`](../validation/wobble.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`wobble` is the next bounded plotting continuation after the shipped `density` slice. The EMBOSS-RS v1 surface computes one deterministic codon-windowed third-base-position variability profile for exactly one coding nucleotide sequence and emits both a stable analytical table and a typed single-series line-plot contract.

## Inputs

This tool accepts exactly one coding nucleotide sequence record. v1 supports DNA and RNA inputs that pass strict coding-sequence validation, treats `--codon-window` and `--codon-step` as positive codon counts, defaults to `--codon-window 11` and `--codon-step 1`, rejects ambiguous or non-coding inputs, and ignores only a terminal stop codon after validation.

## Outputs

The implementation emits a stable analytical table with `sequence_id`, `window_start`, `window_end`, `window_length`, `codon_window_length`, `wobble_positions`, `adenine_fraction`, `cytosine_fraction`, `guanine_fraction`, `thymine_fraction`, `dominant_wobble_fraction`, and `wobble_variability`. The staged v1 line-plot contract remains single-series and plots the analytically derived `wobble_variability` values from the same governed computation path.

## Plotting Integration

Rust does not render figures. The formal contract emitted by `wobble` is the governed handoff to the sister `emboss-r` package, which owns graphical rendering. This shipped `wobble` slice intentionally stays single-series and renderer-agnostic even though the analytical table exposes richer bounded third-position composition detail.

## Current Status

This method is now implemented and exposed through `emboss-rs wobble`. At this task boundary the governed surface includes executable analytical and plot-contract validation, but compared acceptance evidence for the canonical checked-in fixtures has not landed yet.

## Caveats

v1 supports only the single-record bounded wobble profile and does not add Rust-side rendering, multi-series plotting, region-track behavior, or a generalized plotting framework. Additional continuation should remain bounded by the explicit fallback-activation stop conditions already recorded in governance.

## Declared Artifacts

### Coding nucleotide fixture for governed wobble validation

- Artifact ID: `wobble_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/wobble_coding_nucleotide.fasta`
- Notes: Repository-managed coding nucleotide fixture used for deterministic wobble validation.

## Declared Examples

### Compute a deterministic wobble variability profile

- Example ID: `wobble_profile_example`
- Description: Reports deterministic codon-windowed wobble-variability rows from the committed coding nucleotide fixture and emits a governed single-series wobble-variability line-plot contract from the same analytical run.
- Referenced artifacts: `wobble_fixture`
- Parameters:
  - `codon_window` = `3`
  - `codon_step` = `1`
- Expected outputs:
  - `wobble_table`: Wobble variability table (Stable codon-windowed wobble rows with explicit third-position composition and derived variability columns.)
  - `wobble_plot`: Wobble-variability line-plot contract (The governed single-series wobble-variability line-plot contract JSON emitted from the same deterministic analytical run.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS wobble application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/wobble.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `wobble_profile_example`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes
