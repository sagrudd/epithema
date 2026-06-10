# splitsource

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Split annotated records into one fragment per simple source feature

## Document Metadata

- Document ID: `splitsource-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `feature_tools`
- Legacy names: `splitsource`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/splitsource.validation.json`](../validation/splitsource.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`splitsource` divides one annotated sequence record into multiple fragments using `source` features as the split plan. Epithema v1 treats only simple single-span `source` features as eligible, requires at least two such features per record, and emits one unannotated fragment sequence per source interval in source order.

## Inputs

The current interface accepts one annotated EMBL or GenBank input path. Records must contain at least two eligible `source` features, and each eligible source feature must have a simple single-span location.

## Outputs

The tool emits a sequence collection through the shared FASTA output path. Fragment identifiers are derived as `<parent>-source-<n>`, descriptions record the ordinal and interval used, and CLI summaries report the input path, fragment count, and the conservative source-feature selection rule.

## Current Status

This method is implemented and exposed through `epithema splitsource`. Validation currently covers deterministic splitting of a committed GenBank fixture containing two `source` features, along with rejection of records that do not contain multiple eligible source spans.

## Caveats

The first release does not preserve annotation on emitted fragments, does not support complex `source` feature locations, and does not merge or infer fragment metadata beyond conservative identifier and description derivation.

## Declared Artifacts

### Annotated GenBank source fixture

- Artifact ID: `splitsource_gbk_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/splitsource_annotated.gbk`
- Notes: Repository-managed annotated GenBank fixture containing two simple `source` features for deterministic fragment emission.

## Declared Examples

### Emit one fragment per simple source feature

- Example ID: `split_record_by_source_features`
- Description: Runs `splitsource` against a committed annotated GenBank fixture and emits two unannotated FASTA fragments in source-feature order.
- Referenced artifacts: `splitsource_gbk_fixture`
- Expected outputs:
  - `source_fragment_collection`: Source-derived FASTA fragments (A FASTA sequence collection containing two fragments with residues `AAAT` and `GGGC`.)

## Provenance

- Curated by: epithema maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `split_record_by_source_features`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
