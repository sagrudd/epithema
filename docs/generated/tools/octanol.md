# octanol

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report a sliding-window White-Wimley interface-minus-octanol profile and emit a line-plot contract

## Document Metadata

- Document ID: `octanol-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `protein_plots`
- Legacy names: `octanol`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/octanol.validation.json`](../validation/octanol.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`octanol` is the second shipped method in the bounded plotting rework program. The EMBOSS-RS v1 surface computes one deterministic sliding-window White-Wimley interface-minus-octanol profile for exactly one protein sequence and emits both a stable analytical table and a typed single-series line-plot contract.

## Inputs

This tool accepts exactly one protein sequence record. v1 requires supported amino-acid residues, treats `--window` and `--step` as positive residue counts, defaults to `--window 19` and `--step 1`, and rejects sequences shorter than the requested window.

## Outputs

The implementation emits a stable table with `sequence_id`, `window_start`, `window_end`, `window_length`, and `interface_minus_octanol`, plus a typed line-plot contract payload. The x axis uses 1-based window starts and the y axis reports the deterministic White-Wimley interface-minus-octanol value computed from the same governed analytical path.

## Plotting Integration

Rust does not render figures. The formal contract emitted by `octanol` is the governed handoff to the sister `emboss-r` package, which owns graphical rendering. This shipped `octanol` slice intentionally stays inside the existing single-series line-contract seam already proven by `charge`, `pepwindow`, and `hmoment`.

## Current Status

This method is implemented and exposed through `emboss-rs octanol`. Validation now covers stable analytical rows plus compared acceptance evidence for the canonical checked-in line-plot contract emission path, while keeping rendering in the sister `emboss-r` package.

## Caveats

v1 supports only the single-record White-Wimley difference profile and does not add Rust-side rendering, multi-series plotting, or a broader plotting-family revival. Additional plotting methods should reuse the same contract-and-renderer split rather than widening the Rust plotting surface informally.

## Declared Artifacts

### Protein fixture for governed octanol validation

- Artifact ID: `octanol_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/octanol_protein.fasta`
- Notes: Repository-managed protein fixture used for deterministic octanol validation.

### Canonical octanol line-plot contract fixture

- Artifact ID: `octanol_plot_contract`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/octanol_plot_contract.json`
- Notes: Repository-managed canonical line-plot contract fixture emitted by the governed octanol implementation.

## Declared Examples

### Compute a deterministic White-Wimley interface-minus-octanol profile

- Example ID: `octanol_profile_example`
- Description: Reports deterministic sliding-window White-Wimley difference rows from the committed protein fixture and emits a governed single-series line-plot contract from the same analytical run.
- Referenced artifacts: `octanol_fixture`, `octanol_plot_contract`
- Parameters:
  - `window` = `3`
  - `step` = `1`
- Expected outputs:
  - `octanol_table`: White-Wimley interface-minus-octanol table (Stable sliding-window White-Wimley difference rows plus a canonical line-plot contract derived from the same governed output.)
  - `octanol_plot`: White-Wimley line-plot contract (The canonical governed line-plot contract JSON emitted from the same deterministic analytical run.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS octanol application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/octanol.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `octanol_profile_example`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes
