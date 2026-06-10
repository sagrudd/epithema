# feattext

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Render selected features as normalized feature-table text

## Document Metadata

- Document ID: `feattext-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `feature_tools`
- Legacy names: `feattext`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/feattext.validation.json`](../validation/feattext.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`feattext` renders selected features from annotated EMBL or GenBank records as normalized feature-table text. It reuses the shared Epithema feature-selection model and emits a deterministic textual view of the current structured feature representation.

## Inputs

The current v1 tool accepts one annotated EMBL or GenBank input plus optional `--kind`, `--name`, `--qualifier`, and `--strand` selectors. Selectors combine conjunctively, and if none are supplied all features are rendered.

## Outputs

The output is a text report grouped by source record, with an `ID` header, a normalized `FEATURES             Location/Qualifiers` block, and one rendered feature entry per selected feature. Output ordering is stable by source record and feature order.

## Current Status

This method is implemented and exposed through `epithema feattext`. Rust tests currently cover normalized rendering, service invocation, and selector-driven no-match handling against committed annotated fixtures.

## Caveats

The v1 output is intentionally a governed normalized rendering, not a byte-for-byte recovery of the original source flatfile text. Feature names are rendered through the Epithema structured model, and qualifiers are emitted in stable key order.

## Declared Artifacts

### Annotated GenBank fixture

- Artifact ID: `annotated_feature_genbank`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/annotated_feature.gbk`
- Notes: Repository-managed annotated GenBank fixture used for deterministic feattext validation.

## Declared Examples

### Render normalized feature text from an annotated record

- Example ID: `render_normalized_feature_text`
- Description: Renders the committed annotated GenBank fixture into the governed normalized feature-text view.
- Referenced artifacts: `annotated_feature_genbank`
- Expected outputs:
  - `feature_text`: Normalized feature table (A normalized feature-table text block is emitted for the selected features.)
- Legacy reference: EMBOSS feattext application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/feattext.acd`
  - Invocation: `feattext -sequence annotated_feature.gbk -outfile stdout`

## Provenance

- Curated by: epithema maintainers
- Source references:
  - EMBOSS feattext application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/feattext.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `render_normalized_feature_text`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
