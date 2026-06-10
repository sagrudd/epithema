# pepinfo

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report a bounded sliding-window multi-property protein profile and emit a multi-series line-plot contract

## Document Metadata

- Document ID: `pepinfo-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `protein_plots`
- Legacy names: `pepinfo`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/pepinfo.validation.json`](../validation/pepinfo.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`pepinfo` is the third shipped method in the bounded plotting rework program. The Epithema v1 surface computes one deterministic sliding-window multi-property protein profile for exactly one protein sequence and emits both a stable analytical table and a typed multi-series line-plot contract.

## Inputs

This tool accepts exactly one protein sequence record. v1 requires supported amino-acid residues, treats `--window` and `--step` as positive residue counts, defaults to `--window 9` and `--step 1`, and rejects sequences shorter than the requested window.

## Outputs

The implementation emits a stable table with `sequence_id`, `window_start`, `window_end`, `window_length`, `mean_hydropathy`, `mean_residue_mass`, `charged_fraction`, and `polar_fraction`, plus a typed multi-series line-plot contract payload. The x axis uses 1-based window starts and every plotted series is derived from the same governed analytical table.

## Plotting Integration

Rust does not render figures. The formal contract emitted by `pepinfo` is the governed handoff to the sister `epithemaR` package, which owns graphical rendering. This shipped `pepinfo` slice is the first bounded multi-series contract in the plotting rework, but it still stays table-derived, renderer-agnostic, and method-associated.

## Current Status

This method is implemented and exposed through `epithema pepinfo`. Validation now covers stable analytical rows plus compared acceptance evidence for the canonical checked-in multi-series line-plot contract emission path, while keeping rendering in the sister `epithemaR` package.

## Caveats

v1 supports only the single-record bounded `pepinfo` profile and does not add Rust-side rendering, broader plotting-family revival, or generic plotting-framework behavior. Additional plotting work should continue to reuse the same contract-and-renderer split rather than widening the Rust surface informally.

## Declared Artifacts

### Protein fixture for governed pepinfo validation

- Artifact ID: `pepinfo_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/pepinfo_protein.fasta`
- Notes: Repository-managed protein fixture used for deterministic pepinfo validation.

### Canonical pepinfo multi-series line-plot contract fixture

- Artifact ID: `pepinfo_plot_contract`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/pepinfo_plot_contract.json`
- Notes: Repository-managed canonical multi-series line-plot contract fixture emitted by the governed pepinfo implementation.

## Declared Examples

### Compute a deterministic bounded protein profile

- Example ID: `pepinfo_profile_example`
- Description: Reports deterministic sliding-window multi-property rows from the committed protein fixture and emits a governed multi-series line-plot contract from the same analytical run.
- Referenced artifacts: `pepinfo_fixture`, `pepinfo_plot_contract`
- Parameters:
  - `window` = `3`
  - `step` = `1`
- Expected outputs:
  - `pepinfo_table`: Bounded pepinfo profile table (Stable sliding-window multi-property rows derived from the governed analytical path.)
  - `pepinfo_plot`: Pepinfo multi-series line-plot contract (The canonical governed multi-series line-plot contract JSON emitted from the same deterministic analytical run.)

## Provenance

- Curated by: epithema maintainers
- Source references:
  - EMBOSS pepinfo application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/pepinfo.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `pepinfo_profile_example`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes
