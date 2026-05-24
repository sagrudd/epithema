# density

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report a sliding-window nucleotide density profile and emit a line-plot contract

## Document Metadata

- Document ID: `density-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `nucleotide_plots`
- Legacy names: `density`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/density.validation.json`](../validation/density.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`density` is the first shipped Phase 2 method in the bounded plotting rework continuation. The EMBOSS-RS v1 surface computes one deterministic sliding-window nucleotide density profile for exactly one nucleotide sequence and emits both a stable analytical table and a typed single-series line-plot contract.

## Inputs

This tool accepts exactly one nucleotide sequence record. v1 supports DNA and RNA inputs, treats `--window` and `--step` as positive residue counts, defaults to `--window 11` and `--step 1`, and rejects sequences shorter than the requested window.

## Outputs

The implementation emits a stable analytical table with `sequence_id`, `window_start`, `window_end`, `window_length`, `canonical_symbols`, `ambiguous_symbols`, `ignored_gap_symbols`, `adenine_fraction`, `cytosine_fraction`, `guanine_fraction`, `thymine_or_uracil_fraction`, `at_fraction`, and `gc_fraction`. The staged v1 line-plot contract remains single-series and plots the analytically derived `gc_fraction` values from the same governed computation path.

## Plotting Integration

Rust does not render figures. The formal contract emitted by `density` is the governed handoff to the sister `emboss-r` package, which owns graphical rendering. This shipped `density` slice intentionally stays single-series and renderer-agnostic even though the analytical table exposes a richer bounded nucleotide-density surface.

## Current Status

This method is implemented and exposed through `emboss-rs density`. At this task boundary the method is shipped with executable evidence and harvested historical provenance, but canonical checked-in plot-contract comparison and full compared acceptance evidence are still pending.

## Caveats

v1 supports only the single-record bounded density profile and does not add Rust-side rendering, multi-series plotting, or a generalized plotting framework. Additional plotting continuation should remain bounded by the same explicit fallback-activation stop conditions already recorded in governance.

## Declared Artifacts

### Nucleotide fixture for governed density validation

- Artifact ID: `density_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/density_nucleotide.fasta`
- Notes: Repository-managed nucleotide fixture used for deterministic density validation.

## Declared Examples

### Compute a deterministic nucleotide density profile

- Example ID: `density_profile_example`
- Description: Reports deterministic sliding-window nucleotide-density rows from the committed nucleotide fixture and emits a governed single-series GC-fraction line-plot contract from the same analytical run.
- Referenced artifacts: `density_fixture`
- Parameters:
  - `window` = `4`
  - `step` = `1`
- Expected outputs:
  - `density_table`: Nucleotide density table (Stable sliding-window nucleotide-density rows with explicit canonical, ambiguous, and gap-accounting columns.)
  - `density_plot`: GC-fraction line-plot contract (The staged governed line-plot contract JSON emitted from the same deterministic analytical run.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS density application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/density.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `density_profile_example`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes
