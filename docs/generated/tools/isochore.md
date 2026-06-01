# isochore

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report a bounded isochore profile and emit a line-plot contract

## Document Metadata

- Document ID: `isochore-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `nucleotide_plots`
- Legacy names: `isochore`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/isochore.validation.json`](../validation/isochore.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`isochore` is the next bounded plotting continuation after the shipped `wobble` slice. The EMBOSS-RS v1 surface computes one deterministic sliding-window bounded isochore profile for exactly one nucleotide sequence and emits both a stable analytical table and a typed single-series line-plot contract.

## Inputs

This tool accepts exactly one nucleotide sequence record. v1 supports DNA and RNA inputs that pass nucleotide validation, treats `--window` and `--step` as positive residue counts, defaults to `--window 11` and `--step 1`, and rejects non-nucleotide inputs.

## Outputs

The implementation emits a stable analytical table with `sequence_id`, `window_start`, `window_end`, `window_length`, `canonical_symbols`, `ambiguous_symbols`, `ignored_gap_symbols`, `at_fraction`, `gc_fraction`, `gc_percent`, and `isochore_band`. The staged v1 line-plot contract remains single-series and plots the analytically derived `gc_percent` values from the same governed computation path.

## Plotting Integration

Rust does not render figures. The formal contract emitted by `isochore` is the governed handoff to the sister `emboss-r` package, which owns graphical rendering. This shipped `isochore` slice intentionally stays single-series and renderer-agnostic even though the analytical table exposes richer bounded nucleotide-composition detail.

## Current Status

This method is implemented and exposed through `emboss-rs isochore`. Validation now covers stable analytical rows plus compared acceptance evidence for the canonical checked-in GC-percent line-plot contract emission path, while keeping rendering in the sister `emboss-r` package.

## Caveats

v1 supports only the single-record bounded isochore profile and does not add Rust-side rendering, multi-series plotting, segmentation tracks, or a generalized plotting framework. Additional continuation should remain bounded by the explicit fallback-activation stop conditions already recorded in governance.

## Declared Artifacts

### Nucleotide fixture for governed isochore validation

- Artifact ID: `isochore_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/isochore_nucleotide.fasta`
- Notes: Repository-managed nucleotide fixture used for deterministic isochore validation.

### Canonical isochore line-plot contract fixture

- Artifact ID: `isochore_plot_contract`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/isochore_plot_contract.json`
- Notes: Repository-managed canonical GC-percent line-plot contract fixture emitted by the governed isochore implementation.

## Declared Examples

### Compute a deterministic bounded isochore profile

- Example ID: `isochore_profile_example`
- Description: Reports deterministic sliding-window isochore rows from the committed nucleotide fixture and emits a governed single-series GC-percent line-plot contract from the same analytical run.
- Referenced artifacts: `isochore_fixture`, `isochore_plot_contract`
- Parameters:
  - `window` = `4`
  - `step` = `4`
- Expected outputs:
  - `isochore_table`: Isochore analytical table (Stable sliding-window bounded isochore rows with explicit GC, AT, and band columns.)
  - `isochore_plot`: GC-percent line-plot contract (The governed single-series GC-percent line-plot contract JSON emitted from the same deterministic analytical run.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS isochore application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/isochore.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `isochore_profile_example`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes
