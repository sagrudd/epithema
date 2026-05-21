# twofeat

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report neighbouring feature pairs that satisfy conservative selector and distance rules

## Document Metadata

- Document ID: `twofeat-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `feature_tools`
- Legacy names: `twofeat`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/twofeat.validation.json`](../validation/twofeat.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`twofeat` reports neighbouring feature pairs from annotated sequence records. EMBOSS-RS v1 defines neighbours conservatively as adjacent features in source order, supports the existing selector model for the left and right features independently, and emits a governed table report instead of a feature-rewriting payload.

## Inputs

The current interface accepts one annotated EMBL or GenBank input. Left-side selectors use `--a-kind`, `--a-name`, `--a-qualifier`, and `--a-strand`; right-side selectors use the corresponding `--b-*` flags. Optional `--min-range` and `--max-range` constrain the nearest-end gap between adjacent features.

## Outputs

The tool emits a stable table report through the shared result layer. Rows preserve source-record order and adjacent feature-pair order, and include per-feature kind, location, name, nearest-end gap, relation, and strand-relation fields.

## Current Status

This method is implemented and exposed through `emboss-rs twofeat`. Validation currently covers deterministic reporting of one adjacent `gene`/`CDS` pair from a committed annotated fixture, plus distance-filter rejection in the Rust tool layer.

## Caveats

The first release does not attempt the broader historical EMBOSS neighbourhood model. It considers only adjacent features in source order, reports distance from nearest feature bounds, and does not support inter-record pairing, complex interval geometry metrics, or richer pair-scoring rules.

## Declared Artifacts

### Annotated feature GenBank fixture

- Artifact ID: `annotated_feature_gbk`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/annotated_feature.gbk`
- Notes: Repository-managed annotated GenBank fixture containing one `gene` followed by one `CDS`, used for deterministic neighbouring-feature reporting.

## Declared Examples

### Report one neighbouring gene/CDS pair

- Example ID: `report_gene_cds_neighbour_pair`
- Description: Runs `twofeat` against a committed annotated GenBank fixture with explicit left and right feature selectors for `gene` and `cds`.
- Referenced artifacts: `annotated_feature_gbk`
- Parameters:
  - `a-kind` = `gene`
  - `b-kind` = `cds`
- Expected outputs:
  - `neighbouring_feature_pair_report`: Neighbouring feature pair table (A governed table with one row for the adjacent `gene` then `CDS` pair, a gap of `1`, and a `separated` relation.)
- Legacy reference: EMBOSS twofeat application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/twofeat.acd`
  - Invocation: `twofeat -sequence annotated_feature.gbk -atype gene -btype cds -stdout yes`

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `report_gene_cds_neighbour_pair`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes

