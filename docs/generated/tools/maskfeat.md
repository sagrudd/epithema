# maskfeat

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Mask selected simple annotated feature spans in-place while preserving record annotations

## Document Metadata

- Document ID: `maskfeat-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `feature_tools`
- Legacy names: `maskfeat`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/maskfeat.validation.json`](../validation/maskfeat.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`maskfeat` masks sequence spans defined by selected annotated features from EMBL or GenBank inputs using the shared EMBOSS-RS feature-selection and interval-masking path. The original record is retained, matching simple feature spans are masked in place, and feature annotations remain attached to the resulting payload.

## Inputs

The current v1 tool accepts annotated EMBL or GenBank input plus optional selector flags. Supported selectors are `--kind`, `--name`, `--qualifier`, and `--strand`. When more than one selector is supplied they are combined conjunctively. If no selector flags are provided, all features in each input record are selected.

## Outputs

The output preserves source record order and emits masked full-length records in FASTA form while retaining original feature annotations in the structured payload. Overlapping or adjacent selected simple spans are masked deterministically in place, so repeated coverage does not change the final result beyond replacing covered residues with the chosen mask symbol.

## Masking Policy

The default mask symbol is `N` for nucleotide and unknown-molecule records and `X` for protein records. A custom single-character mask can be supplied with `--mask-char` only when that symbol is valid for the target alphabet. Selected feature spans are not removed or rebased; they remain attached to the original coordinate system of the masked record.

## Current Status

This method is implemented and exposed through `emboss-rs maskfeat`. Validation currently covers masking a selected gene feature from a committed annotated GenBank fixture. Rust service and core tests also cover masking all selected spans in one record, no-match handling, invalid mask-symbol handling, and conservative failure on unsupported complex feature locations.

## Caveats

The v1 scope supports only simple single-span feature locations. Joined, compound, or otherwise complex locations fail clearly rather than being partially masked. If no selected features are found across the input, the tool fails with an explicit validation error instead of silently returning unchanged records.

## Declared Artifacts

### Annotated GenBank fixture

- Artifact ID: `annotated_feature_genbank`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/annotated_feature.gbk`
- Notes: Repository-managed annotated GenBank fixture used for deterministic maskfeat validation.

## Declared Examples

### Mask a selected gene feature in-place on an annotated record

- Example ID: `mask_selected_gene_feature`
- Description: Selects the `gene` feature from a small annotated GenBank fixture and masks that span with the default nucleotide mask symbol while preserving the source record length and annotations.
- Referenced artifacts: `annotated_feature_genbank`
- Parameters:
  - `kind` = `gene`
- Expected outputs:
  - `feature_masked_sequences`: Feature-masked sequence records (The source sequence is returned at full length with the selected feature span replaced by masking symbols in place.)
- Legacy reference: EMBOSS maskfeat application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/maskfeat.acd`
  - Invocation: `maskfeat -sequence annotated_feature.gbk -type gene -outseq stdout`

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `mask_selected_gene_feature`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
