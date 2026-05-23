# hmoment

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report a sliding-window protein hydrophobic-moment profile and emit a line-plot contract

## Document Metadata

- Document ID: `hmoment-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `protein_plots`
- Legacy names: `hmoment`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/hmoment.validation.json`](../validation/hmoment.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`hmoment` is the first shipped method in the bounded plotting rework program. The EMBOSS-RS v1 surface computes one deterministic sliding-window protein hydrophobic-moment profile for exactly one protein sequence and emits both a stable analytical table and a typed single-series line-plot contract.

## Inputs

This tool accepts exactly one protein sequence record. v1 requires supported amino-acid residues, treats `--window` and `--step` as positive residue counts, defaults to `--window 11`, `--step 1`, and `--angle-degrees 100`, and rejects sequences shorter than the requested window.

## Outputs

The implementation emits a stable table with `sequence_id`, `window_start`, `window_end`, `window_length`, and `hydrophobic_moment`, plus a typed line-plot contract payload. The x axis uses 1-based window starts and the y axis reports the deterministic hydrophobic moment computed from the same governed analytical path.

## Plotting Integration

Rust does not render figures. The formal contract emitted by `hmoment` is the governed handoff to the sister `emboss-r` package, which owns graphical rendering. This first shipped `hmoment` slice intentionally stays inside the existing single-series line-contract seam already proven by `charge` and `pepwindow`.

## Current Status

This method is implemented and exposed through `emboss-rs hmoment`. Validation now covers stable analytical rows plus compared acceptance evidence for the canonical checked-in line-plot contract emission path, while keeping rendering in the sister `emboss-r` package.

## Caveats

v1 supports only the single-record `hmoment` profile and does not add Rust-side rendering, multi-series plotting, or a broader plotting-family revival. Additional plotting methods should reuse the same contract-and-renderer split rather than widening the Rust plotting surface informally.

## Declared Artifacts

### Protein fixture for governed hmoment validation

- Artifact ID: `hmoment_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/hmoment_protein.fasta`
- Notes: Repository-managed protein fixture used for deterministic hmoment validation.

### Canonical hmoment line-plot contract fixture

- Artifact ID: `hmoment_plot_contract`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/hmoment_plot_contract.json`
- Notes: Repository-managed canonical line-plot contract fixture emitted by the governed hmoment implementation.

## Declared Examples

### Compute a deterministic protein hydrophobic-moment profile

- Example ID: `hmoment_profile_example`
- Description: Reports deterministic sliding-window hydrophobic-moment rows from the committed protein fixture and emits a governed single-series line-plot contract from the same analytical run.
- Referenced artifacts: `hmoment_fixture`, `hmoment_plot_contract`
- Parameters:
  - `window` = `4`
  - `step` = `1`
  - `angle-degrees` = `100`
- Expected outputs:
  - `hmoment_table`: Hydrophobic-moment profile table (Stable sliding-window hydrophobic-moment rows plus a canonical line-plot contract derived from the same governed output.)
  - `hmoment_plot`: Hydrophobic-moment line-plot contract (The canonical governed line-plot contract JSON emitted from the same deterministic analytical run.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS hmoment application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/hmoment.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `hmoment_profile_example`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes
