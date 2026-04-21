# extractfeat

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Extract selected simple feature-defined sequence regions into rebased output records

## Document Metadata

- Document ID: `extractfeat-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `feature_tools`
- Legacy names: `extractfeat`

## Overview

`extractfeat` extracts sequence regions defined by selected annotated features from EMBL or GenBank inputs using the shared EMBOSS-RS feature-selection and extraction foundations. Each selected simple feature span becomes one output record, and the copied feature is rebased onto the local extracted coordinate system.

## Inputs

The current v1 tool accepts annotated EMBL or GenBank input plus optional feature-selection flags. Supported selectors are `--kind`, `--name`, `--qualifier`, and `--strand`, which combine conjunctively when more than one is supplied. If no selector flags are provided, all annotated features are considered.

## Outputs

One output sequence record is emitted per selected simple feature in stable source and feature order. Output identifiers are derived deterministically as `<source>:<start>-<end>:<feature-name-or-kind>`, extracted residues preserve the source molecule kind and metadata, and the copied feature location is rebased so the extracted span starts at local coordinate 1.

## Current Status

This method is implemented and exposed through `emboss-rs extractfeat`. Validation currently covers gene extraction from a committed annotated GenBank fixture. Rust service and core tests also cover qualifier-based selection, no-match handling, coordinate rebasing, and reverse-strand nucleotide extraction through the shared feature-extraction layer.

## Caveats

The v1 scope supports only simple single-span feature locations. Joined, compound, or otherwise complex feature locations fail clearly rather than being partially interpreted. Reverse-strand extraction is supported only where the shared core can reverse-complement the underlying molecule correctly, and unannotated inputs or selector combinations with no matches fail with explicit validation errors.

## Declared Artifacts

### Annotated GenBank fixture

- Artifact ID: `annotated_feature_genbank`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/annotated_feature.gbk`
- Notes: Repository-managed annotated GenBank fixture used for deterministic extractfeat validation.

## Declared Examples

### Extract a selected gene feature into a rebased record

- Example ID: `extract_selected_gene_feature`
- Description: Selects the `gene` feature from a small annotated GenBank fixture and emits one rebased extracted sequence record.
- Referenced artifacts: `annotated_feature_genbank`
- Parameters:
  - `kind` = `gene`
- Expected outputs:
  - `extracted_feature_sequences`: Feature-defined extracted sequences (The selected `gene` span is emitted as its own sequence record with rebased copied feature coordinates.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Validation Intent

- Required examples: `extract_selected_gene_feature`
- Compare against legacy: no
- Require provenance capture: yes

