# inforesidue

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report deterministic metadata for one canonical amino-acid residue

## Document Metadata

- Document ID: `inforesidue-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_stats`
- Legacy names: `inforesidue`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/inforesidue.validation.json`](../validation/inforesidue.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`inforesidue` reports one deterministic metadata row for a single canonical amino-acid residue. The EMBOSS-RS v1 implementation focuses on stable naming, mass, hydropathy, and coarse biochemical classes rather than reproducing multiple historical report styles.

## Inputs

The current interface accepts exactly one canonical amino-acid residue symbol. The v1 surface supports the standard twenty residues and rejects ambiguous symbols such as `X`.

## Outputs

The tool emits a stable one-row table with columns `residue`, `three_letter`, `name`, `charge_class`, `polarity_class`, `average_mass`, and `hydropathy`.

## Property Model

Average mass uses the same residue-mass table used by `pepstats`. Hydropathy uses the same Kyte-Doolittle scale used by `pepwindow`. `charge_class` and `polarity_class` are governed coarse classes rather than historical AAINDEX identifiers.

## Current Status

This method is implemented and exposed through `emboss-rs inforesidue`. Validation currently covers canonical-residue lookup, stable one-row table emission, and rejection of unsupported residue symbols.

## Caveats

The first release does not expose the broader historical biochemical or structural annotation surface. It is a stable lookup over one governed residue-property table.

## Declared Artifacts

### Inforesidue lysine lookup case

- Artifact ID: `lysine_case`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-testkit/tests/fixtures/autodoc/inforesidue_lookup_lysine_case.md`
- Notes: Repository-managed case note describing the canonical lysine lookup example.

## Declared Examples

### Look up canonical lysine

- Example ID: `lookup_lysine`
- Description: Reports the stable one-row residue table for lysine.
- Referenced artifacts: `lysine_case`
- Parameters:
  - `residue` = `K`
- Expected outputs:
  - `inforesidue_table`: Residue metadata row (A stable one-row amino-acid property table.)
- Legacy reference: EMBOSS inforesidue application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/inforesidue.acd`
  - Invocation: `inforesidue -residue K`

## Provenance

- Curated by: OpenAI Codex
- Source references:
  - EMBOSS inforesidue application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/inforesidue.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `lookup_lysine`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
