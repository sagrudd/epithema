# cons

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Simple majority consensus from an alignment

## Document Metadata

- Document ID: `cons-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `alignment_analysis`
- Legacy names: `cons`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/cons.validation.json`](../validation/cons.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`cons` derives one simple consensus sequence from a single aligned FASTA or Stockholm alignment. The current Epithema implementation ignores gaps when tallying a column, chooses the unique majority non-gap residue when one exists, and falls back conservatively to `N` for nucleotide ambiguity or `X` for protein ambiguity when no unique winner remains.

## Inputs

The current v1 interface accepts exactly one local aligned FASTA or Stockholm alignment path.

## Outputs

The result is one consensus sequence record emitted through the shared sequence result path. The consensus identifier is currently a governed placeholder derived by the tool implementation.

## Current Status

This method is implemented and exposed through `epithema cons`. Current Rust service coverage exercises the committed three-row Stockholm fixture and locks down the simple consensus sequence `ACNGT`.

## Caveats

The first release keeps the consensus policy deliberately simple. It does not expose configurable plurality thresholds, weighted residue counts, or richer historical EMBOSS reporting around consensus quality.

## Declared Artifacts

### Multiple-alignment Stockholm fixture

- Artifact ID: `multiple_alignment_stockholm`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/multiple_alignment.sto`
- Notes: Repository-managed Stockholm alignment fixture used to validate simple consensus derivation.

## Declared Examples

### Derive a simple consensus from a three-row Stockholm alignment

- Example ID: `derive_simple_consensus_from_stockholm_alignment`
- Description: Loads the committed multiple-alignment Stockholm fixture and reports the simple consensus sequence under the current majority-rule policy.
- Referenced artifacts: `multiple_alignment_stockholm`
- Expected outputs:
  - `simple_consensus_sequence`: Simple consensus sequence (A consensus sequence record whose residues are `ACNGT` for the committed fixture.)

## Provenance

- Curated by: epithema maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `derive_simple_consensus_from_stockholm_alignment`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
