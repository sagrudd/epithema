# pepwindow

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report a sliding-window protein hydropathy profile and emit a line-plot contract

## Document Metadata

- Document ID: `pepwindow-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `protein_plots`
- Legacy names: `pepwindow`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/pepwindow.validation.json`](../validation/pepwindow.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`pepwindow` is the next governed plot-family method after `charge`. The Epithema v1 surface computes one deterministic Kyte-Doolittle sliding-window hydropathy profile for exactly one protein sequence and emits both a stable analytical table and a typed line-plot contract.

## Inputs

This tool accepts exactly one protein sequence record. v1 requires standard supported amino-acid residues, treats `--window` and `--step` as positive residue counts, defaults to `--window 19` and `--step 1`, and rejects sequences shorter than the requested window.

## Outputs

The implementation emits a stable table with `sequence_id`, `window_start`, `window_end`, `window_length`, and `mean_hydropathy`, plus a typed line-plot contract payload. The x axis uses 1-based window starts and the y axis reports the mean Kyte-Doolittle score across each window.

## Plotting Integration

Rust does not render figures. The formal contract emitted by `pepwindow` is the governed handoff to the sister `epithemaR` package, which owns graphical rendering for plot-capable families. In this prompt the Rust-side contract is implemented and validated; method-specific R rendering remains a follow-on task in `epithemaR`.

## Current Status

This method is implemented and exposed through `epithema pepwindow`. Validation now covers stable analytical window rows plus compared acceptance evidence for the canonical checked-in line-plot contract emission path, alongside unsupported-residue rejection.

## Caveats

v1 supports only the single-record `pepwindow` profile and does not implement `pepwindowall` or any Rust-side rendering. Ambiguous or unsupported residues are rejected instead of being approximated.

## Declared Artifacts

### Protein fixture for governed pepwindow validation

- Artifact ID: `pepwindow_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/pepwindow_protein.fasta`
- Notes: Repository-managed protein fixture used for deterministic pepwindow hydropathy validation.

### Canonical pepwindow line-plot contract fixture

- Artifact ID: `pepwindow_plot_contract`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/pepwindow_plot_contract.json`
- Notes: Repository-managed canonical line-plot contract fixture emitted by the governed pepwindow implementation.

## Declared Examples

### Compute a deterministic protein hydropathy profile and emit a line-plot contract

- Example ID: `pepwindow_profile_example`
- Description: Reports deterministic sliding-window hydropathy rows from the committed protein fixture and emits the canonical governed line-plot contract.
- Referenced artifacts: `pepwindow_fixture`, `pepwindow_plot_contract`
- Parameters:
  - `window` = `5`
  - `step` = `2`
- Expected outputs:
  - `pepwindow_table`: Hydropathy profile table (Stable sliding-window hydropathy rows plus a canonical line-plot contract derived from the same governed output.)
  - `pepwindow_plot`: Hydropathy line-plot contract (The canonical governed line-plot contract JSON emitted from the same deterministic analytical run.)

## Provenance

- Curated by: epithema maintainers
- Source references:
  - EMBOSS pepwindow application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/pepwindow.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `pepwindow_profile_example`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes
