# codcopy

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Normalize coding-sequence or codon-profile input into a reusable codon profile

## Document Metadata

- Document ID: `codcopy-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `codon_tools`
- Legacy names: `codcopy`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/codcopy.validation.json`](../validation/codcopy.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`codcopy` normalizes a coding-sequence input or an existing normalized codon-profile input into the governed codon-usage profile model. It can also write that profile back out as a tab-separated reusable profile file for later `cai` or `codcmp` runs.

## Inputs

The current v1 interface accepts one local source input and an optional `--profile-out` path. The source may be either a strict coding-sequence file or an existing normalized profile TSV with the governed codon-profile header.

## Outputs

The result is a normalized codon-usage profile. When `--profile-out` is supplied, the profile is written as tab-separated text with the stable header `codon\tamino_acid\tcount\tfrequency\tweight` so it can be reused directly by `cai` and `codcmp`.

## Current Status

This method is implemented and exposed through `epithema codcopy`. Rust service coverage writes a temporary codon-profile TSV from a committed coding fixture and then proves downstream interoperability by reusing that profile successfully in a `cai` invocation.

## Caveats

The first release is local-only and strict. Coding inputs are validated in the same way as the rest of the codon-analysis cohort, and profile inputs must use the governed TSV header and five-column row format exactly. This method does not attempt format inference beyond those two supported source shapes.

## Declared Artifacts

### Codon-reference FASTA fixture

- Artifact ID: `codon_reference_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/codon_reference.fasta`
- Notes: Repository-managed coding-reference FASTA fixture used to validate normalized profile generation.

## Declared Examples

### Normalize a coding fixture into a reusable codon-profile TSV

- Example ID: `normalize_coding_input_into_reusable_profile`
- Description: Loads the committed coding-reference FASTA fixture, derives the governed codon-usage profile, and writes it as reusable TSV output for later `cai` or `codcmp` runs.
- Referenced artifacts: `codon_reference_fasta`
- Parameters:
  - `profile_out` = `<temporary-profile.tsv>`
- Expected outputs:
  - `normalized_codon_profile`: Normalized codon-profile TSV (A governed codon-profile TSV with the stable five-column header and weights suitable for later `cai` or `codcmp` input.)

## Provenance

- Curated by: epithema maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `normalize_coding_input_into_reusable_profile`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
