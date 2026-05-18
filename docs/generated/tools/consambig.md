# consambig

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Ambiguity-aware consensus from an alignment

## Document Metadata

- Document ID: `consambig-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `alignment_analysis`
- Legacy names: `consambig`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/consambig.validation.json`](../validation/consambig.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`consambig` derives an ambiguity-aware consensus sequence from a single aligned FASTA or Stockholm alignment. The current EMBOSS-RS implementation ignores gaps when tallying a column, emits IUPAC nucleotide ambiguity symbols when multiple exact nucleotide bases remain, and uses `X` for ambiguous protein columns.

## Inputs

The current v1 interface accepts exactly one local aligned FASTA or Stockholm alignment path.

## Outputs

The result is one ambiguity-aware consensus sequence record emitted through the shared sequence result path. CLI output follows the standard sequence-output rendering path.

## Current Status

This method is implemented and exposed through `emboss-rs consambig`. Current Rust service coverage exercises the committed three-row Stockholm fixture and locks down the ambiguity-aware consensus sequence `ACYGT`.

## Caveats

The first release only covers the current built-in ambiguity policy. It does not expose alternative consensus alphabets, configurable thresholds, or richer legacy reporting fields.

## Declared Artifacts

### Multiple-alignment Stockholm fixture

- Artifact ID: `multiple_alignment_stockholm`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/multiple_alignment.sto`
- Notes: Repository-managed Stockholm alignment fixture used to validate ambiguity-aware consensus derivation.

## Declared Examples

### Derive an ambiguity-aware consensus from a three-row Stockholm alignment

- Example ID: `derive_ambiguity_aware_consensus_from_stockholm_alignment`
- Description: Loads the committed multiple-alignment Stockholm fixture and reports the ambiguity-aware consensus sequence under the current IUPAC-aware policy.
- Referenced artifacts: `multiple_alignment_stockholm`
- Expected outputs:
  - `ambiguity_consensus_sequence`: Ambiguity-aware consensus sequence (A consensus sequence record whose residues are `ACYGT` for the committed fixture.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `derive_ambiguity_aware_consensus_from_stockholm_alignment`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes

