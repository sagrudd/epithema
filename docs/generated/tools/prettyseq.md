# prettyseq

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Render nucleotide records with a paired translated text view

## Document Metadata

- Document ID: `prettyseq-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `translation_tools`
- Legacy names: `prettyseq`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/prettyseq.validation.json`](../validation/prettyseq.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`prettyseq` renders a deterministic text report that shows nucleotide rows and their translated amino-acid rows together. The Epithema v1 surface is intentionally small: one chosen forward frame, stable plain-text layout, and no attempt to reproduce the full historical pretty-printing parameter surface.

## Inputs

The current interface accepts one nucleotide input path plus optional `--frame <1|2|3>` and `--width <positive-count>`. Translation uses the shared standard genetic code, and trailing partial codons are ignored.

## Outputs

The primary payload is a stable text report. Each record is rendered with a header line, one `FRAME N` line, one or more nucleotide rows prefixed with `NT`, and the corresponding amino-acid rows prefixed with `AA`.

## Legacy Context

This acceptance anchor keeps one historical-style `prettyseq` invocation in view and compares the Epithema text-report payload against a committed expected output. The governed comparison validates layout stability for the frame-1 committed coding fixture.

## Current Status

This method is implemented and exposed through `epithema prettyseq`. Current validation covers deterministic frame-1 rendering, stable line layout, and clear rejection of unsupported frame or input combinations.

## Caveats

The first release supports forward frames only and emits a governed plain-text layout rather than attempting byte-for-byte parity with historical EMBOSS formatting or richer annotation-aware sequence displays.

## Declared Artifacts

### Pretty sequence nucleotide fixture

- Artifact ID: `checktrans_nucleotide_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/checktrans_nucleotide.fasta`
- Notes: Repository-managed coding-sequence fixture used for deterministic prettyseq text rendering.

## Declared Examples

### Render a frame-1 nucleotide and amino-acid text report

- Example ID: `render_forward_frame_report`
- Description: Renders the committed coding-sequence fixture with forward frame-1 translation and the governed Epithema plain-text layout.
- Referenced artifacts: `checktrans_nucleotide_fixture`
- Parameters:
  - `frame` = `1`
- Expected outputs:
  - `prettyseq_text_report`: Deterministic text report (A stable text rendering containing nucleotide and amino-acid rows for the committed coding fixture.)
- Legacy reference: EMBOSS prettyseq application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/prettyseq.acd`
  - Invocation: `prettyseq -sequence checktrans_nucleotide.fasta -frame 1 -stdout yes`

## Provenance

- Curated by: epithema maintainers
- Source references:
  - EMBOSS prettyseq application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/prettyseq.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `render_forward_frame_report`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes
