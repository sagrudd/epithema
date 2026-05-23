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

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/extractfeat.validation.json`](../validation/extractfeat.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`extractfeat` extracts sequence regions defined by selected annotated features from EMBL or GenBank inputs using the shared EMBOSS-RS feature-selection and extraction foundations. Each selected simple feature span becomes one output record, and the copied feature is rebased onto the local extracted coordinate system.

## Inputs

The current v1 tool accepts annotated EMBL or GenBank input plus optional feature-selection flags. Supported selectors are `--kind`, `--name`, `--qualifier`, and `--strand`, which combine conjunctively when more than one is supplied. If no selector flags are provided, all annotated features are considered.

## Outputs

One output sequence record is emitted per selected simple feature in stable source and feature order. Output identifiers are derived deterministically as `<source>:<start>-<end>:<feature-name-or-kind>`, extracted residues preserve the source molecule kind and metadata, and the copied feature location is rebased so the extracted span starts at local coordinate 1.

## Legacy Context

This acceptance anchor keeps one historical-style `extractfeat` gene extraction example in view and compares the EMBOSS-RS FASTA payload against a committed expected output. The comparison is intentionally limited to the governed simple-feature path rather than broader historical location semantics.

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
- Legacy reference: EMBOSS extractfeat application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/extractfeat.acd`
  - Invocation: `extractfeat -sequence annotated_feature.gbk -type gene -outseq stdout`

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS extractfeat application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/extractfeat.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `extract_selected_gene_feature`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes
