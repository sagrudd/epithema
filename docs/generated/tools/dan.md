# dan

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report conservative whole-sequence or sliding-window melting estimates

## Document Metadata

- Document ID: `dan-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_stats`
- Legacy names: `dan`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/dan.validation.json`](../validation/dan.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`dan` reports conservative nucleic-acid melting estimates for one or more nucleotide records. The EMBOSS-RS v1 surface keeps the model intentionally narrow: one whole-record row by default, optional sliding windows through `--window` and `--step`, canonical residues only, and one simple deterministic melting-temperature estimate.

## Inputs

The current interface accepts one local nucleotide input path plus optional windowing flags. DNA and RNA records are supported. Non-nucleotide records, ambiguous residues, and gap symbols are rejected because the first-release melting model only supports canonical A/C/G/T/U sequences.

## Outputs

The tool emits a stable table report with columns `record`, `window_index`, `start`, `end`, `length`, `gc_percent`, and `tm_celsius`. Coordinates are user-facing 1-based inclusive start/end values. When no window is supplied, each record yields exactly one row spanning the full sequence.

## Thermodynamic Model

The first release uses a simple deterministic hybrid estimate: short windows below 14 residues use the Wallace rule `2*(A+T/U) + 4*(G+C)`, while longer windows use a basic GC-length estimate `64.9 + 41*(GC - 16.4)/length`. Salt concentration, formamide, mismatch adjustment, thermodynamic DeltaG/DeltaH/DeltaS reporting, and plot output are deferred.

## Current Status

This method is implemented and exposed through `emboss-rs dan`. Validation currently covers whole-record reporting, sliding-window reporting, RNA acceptance, canonical-residue enforcement, and deterministic service-level table output.

## Caveats

The first release is intentionally much narrower than historical EMBOSS `dan`. It does not implement the broader concentration/product/thermodynamic option surface, and it does not emit a plot contract in this prompt.

## Declared Artifacts

### Three-record FASTA fixture

- Artifact ID: `three_record_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/three_records.fasta`
- Notes: Repository-managed canonical DNA fixture used to validate deterministic whole-record and sliding-window melting estimates.

## Declared Examples

### Report sliding-window melting estimates

- Example ID: `report_sliding_window_tm`
- Description: Scans a committed DNA fixture with a size-2 window and reports one row per window with GC percentage and estimated melting temperature.
- Referenced artifacts: `three_record_fasta`
- Parameters:
  - `--window` = `2`
- Expected outputs:
  - `dan_table`: Melting-estimate table (A stable tabular report with one row per whole-record or sliding-window interval and a conservative melting estimate.)
- Legacy reference: EMBOSS dan application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/dan.acd`
  - Invocation: `dan -sequence three_records.fasta -windowsize 2 -outfile stdout`

## Provenance

- Curated by: OpenAI Codex
- Source references:
  - EMBOSS dan application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/dan.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `report_sliding_window_tm`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
