# banana

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report a bounded B-DNA bendability profile and emit a line-plot contract

## Document Metadata

- Document ID: `banana-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `nucleotide_plots`
- Legacy names: `banana`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/banana.validation.json`](../validation/banana.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`banana` is the next bounded plotting continuation after the shipped `isochore` slice. The EMBOSS-RS v1 surface computes one deterministic per-base B-DNA bendability and curvature profile for exactly one nucleotide sequence and emits both a stable analytical table and a typed single-series line-plot contract.

## Inputs

This tool accepts exactly one nucleotide sequence record. v1 supports canonical DNA-like inputs, treats `U` as `T`, rejects ambiguous residues, and does not yet expose alternate legacy angle tables through the governed shipped surface.

## Outputs

The implementation emits a stable analytical table with `sequence_id`, `position`, `residue`, `local_bend`, and `curvature`. Edge positions where the bounded historical model does not define bend or curvature remain blank in the table. The staged v1 line-plot contract remains single-series and plots only the analytically derived `curvature` values at positions where curvature is defined.

## Plotting Integration

Rust does not render figures. The formal contract emitted by `banana` is the governed handoff to the sister `emboss-r` package, which owns graphical rendering. This shipped `banana` slice intentionally stays single-series and renderer-agnostic even though the analytical table exposes both local bend and curvature signals.

## Current Status

This method is implemented and exposed through `emboss-rs banana`. Validation now covers stable analytical rows plus compared acceptance evidence for the canonical checked-in curvature line-plot contract emission path, while keeping rendering in the sister `emboss-r` package.

## Caveats

v1 supports only the single-record bounded default-angle banana profile and does not add Rust-side rendering, multi-series plotting, alternate angle-file selection, or a generalized plotting framework. Additional continuation should remain bounded by the explicit fallback-activation stop conditions already recorded in governance.

## Declared Artifacts

### Nucleotide fixture for governed banana validation

- Artifact ID: `banana_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/banana_nucleotide.fasta`
- Notes: Repository-managed nucleotide fixture used for deterministic banana validation.

### Canonical banana line-plot contract fixture

- Artifact ID: `banana_plot_contract`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/banana_plot_contract.json`
- Notes: Repository-managed canonical curvature line-plot contract fixture emitted by the governed banana implementation.

## Declared Examples

### Compute a deterministic bounded banana profile

- Example ID: `banana_profile_example`
- Description: Reports deterministic per-base bendability and curvature rows from the committed nucleotide fixture and emits a governed single-series curvature line-plot contract from the same analytical run.
- Referenced artifacts: `banana_fixture`, `banana_plot_contract`
- Expected outputs:
  - `banana_table`: Banana analytical table (Stable per-base banana rows with explicit local-bend and curvature columns.)
  - `banana_plot`: Curvature line-plot contract (The governed single-series curvature line-plot contract JSON emitted from the same deterministic analytical run.)
- Legacy reference: EMBOSS banana application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/banana.acd`
  - Invocation: `banana -sequence banana_nucleotide.fasta -graph data`

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS banana application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/banana.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `banana_profile_example`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes
