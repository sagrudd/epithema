# aligncopypair

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Copy a single pairwise alignment unchanged and reject non-pairwise inputs

## Document Metadata

- Document ID: `aligncopypair-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `alignment_tools`
- Legacy names: `aligncopypair`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/aligncopypair.validation.json`](../validation/aligncopypair.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`aligncopypair` copies one pairwise alignment through the shared alignment IO path while enforcing the pairwise constraint explicitly. It is useful when a workflow wants to normalize or inspect pairwise-alignment inputs without silently accepting larger multiple alignments.

## Inputs

The current v1 interface accepts exactly one local aligned FASTA or Stockholm alignment path. The input must contain exactly two alignment rows.

## Outputs

The result is the normalized pairwise alignment payload. CLI rendering follows the standard Epithema alignment output path and emits Stockholm by default.

## Current Status

This method is implemented and exposed through `epithema aligncopypair`. Current Rust service coverage exercises the committed pairwise fixture for the success path and a separate multiple-alignment fixture for the explicit rejection path.

## Caveats

Any input with fewer or more than two alignment rows fails validation instead of being coerced. The first release does not attempt row-pair selection from larger multiple alignments.

## Declared Artifacts

### Pairwise Stockholm fixture

- Artifact ID: `pairwise_alignment_stockholm`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/pairwise_alignment.sto`
- Notes: Repository-managed two-row Stockholm alignment fixture used for the success path.

### Multiple-alignment Stockholm fixture

- Artifact ID: `multiple_alignment_stockholm`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/multiple_alignment.sto`
- Notes: Repository-managed three-row Stockholm alignment fixture used to document the non-pairwise rejection boundary.

## Declared Examples

### Copy a pairwise Stockholm alignment

- Example ID: `copy_pairwise_alignment_stockholm`
- Description: Loads the committed two-row Stockholm alignment fixture and returns it unchanged through the alignment result path.
- Referenced artifacts: `pairwise_alignment_stockholm`
- Expected outputs:
  - `pairwise_alignment_payload`: Pairwise alignment payload (A two-row copied alignment with the original pairwise column layout preserved.)

## Provenance

- Curated by: epithema maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `copy_pairwise_alignment_stockholm`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
