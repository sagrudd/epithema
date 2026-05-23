# featreport

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report selected features as a stable tabular summary

## Document Metadata

- Document ID: `featreport-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `feature_tools`
- Legacy names: `featreport`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/featreport.validation.json`](../validation/featreport.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`featreport` summarizes selected features from annotated EMBL or GenBank records into a stable, machine-friendly table. It reuses the shared EMBOSS-RS feature-selection seam and preserves source-record order followed by feature order.

## Inputs

The current v1 tool accepts one annotated EMBL or GenBank input plus optional `--kind`, `--name`, `--qualifier`, and `--strand` selectors. Selectors combine conjunctively when more than one is supplied, and if none are supplied all features are reported.

## Outputs

The output is a stable table report with one row per selected feature. v1 exposes record identifier, feature kind, normalized location string, 1-based bounds, strand, optional name, and qualifier count.

## Current Status

This method is implemented and exposed through `emboss-rs featreport`. Rust tests currently cover stable ordering, service invocation, and no-match handling through the shared feature-selection and reporting path.

## Caveats

The v1 table is intentionally summary-oriented rather than a full lossless feature-table projection. Multi-span features are reported through a normalized location string, and the report does not attempt to preserve source flatfile formatting.

## Declared Artifacts

### Annotated GenBank fixture

- Artifact ID: `annotated_feature_genbank`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/annotated_feature.gbk`
- Notes: Repository-managed annotated GenBank fixture used for deterministic featreport validation.

## Declared Examples

### Report all features from an annotated record

- Example ID: `report_all_features`
- Description: Summarizes the two features present in the committed annotated GenBank fixture.
- Referenced artifacts: `annotated_feature_genbank`
- Expected outputs:
  - `feature_report`: Feature report (One table row is emitted per feature in stable input order.)
- Legacy reference: EMBOSS featreport application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/featreport.acd`
  - Invocation: `featreport -sequence annotated_feature.gbk -outfile stdout`

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS featreport application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/featreport.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `report_all_features`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
