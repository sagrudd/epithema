# charge

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report a sliding-window protein charge profile and emit a line-plot contract

## Document Metadata

- Document ID: `charge-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `protein_plots`
- Legacy names: `charge`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/charge.validation.json`](../validation/charge.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`charge` is the first production plotting vertical slice in EMBOSS-RS. It was chosen because the analytical output is narrow, deterministic, and plot-ready without hidden biological heuristics: one protein input produces a sliding-window numeric profile with a single numeric x axis and y axis.

## Inputs

This tool accepts exactly one protein sequence record. v1 uses a fixed residue-charge model in Rust, requires protein-compatible residues, and treats the window and step arguments as positive counts over the original sequence.

## Outputs

The implementation emits two coordinated outputs: a structured analytical table of window starts, ends, lengths, and mean charges, plus a typed line-plot contract payload. The plot contract is consumed by the sister `emboss-r` package, which owns rendering.

## Plotting Integration

Rust does not render figures. The formal contract emitted by `charge` is the canonical handoff to R. In `emboss-r`, `charge_profile()` returns a structured result object carrying both the analytical data frame and the parsed plot contract, and `plot()` or `render_charge_plot()` renders the governed line plot with `ggplot2`.

## Current Status

This method is implemented and exposed through `emboss-rs charge`. Rust service tests validate the analytical result and the canonical checked-in plot contract fixture, and the R package consumes the same contract for the first production end-to-end plotting path.

## Caveats

v1 supports the charge profile only for one protein record at a time. Rendering remains intentionally R-owned, so non-graphical Rust and CLI workflows remain usable without R. Additional plot-capable methods should follow this contract-and-renderer pattern rather than adding Rust-side graphics code.

## Declared Artifacts

### Protein fixture for governed charge validation

- Artifact ID: `charge_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/charge_protein.fasta`
- Notes: Repository-managed protein fixture used for deterministic charge-profile validation.

### Canonical charge line-plot contract fixture

- Artifact ID: `charge_plot_contract`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/charge_plot_contract.json`
- Notes: Repository-managed canonical line-plot contract fixture emitted by the governed charge implementation.

## Declared Examples

### Compute a deterministic protein charge profile and emit a line-plot contract

- Example ID: `charge_profile_example`
- Description: Reports deterministic sliding-window charge rows from the committed protein fixture and emits the canonical governed line-plot contract.
- Referenced artifacts: `charge_fixture`, `charge_plot_contract`
- Parameters:
  - `window` = `5`
  - `step` = `1`
- Expected outputs:
  - `charge_profile_table`: Charge profile table (Stable sliding-window charge rows plus a canonical line-plot contract derived from the same governed output.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS charge application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/charge.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `charge_profile_example`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes

