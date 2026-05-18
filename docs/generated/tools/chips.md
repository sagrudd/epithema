# chips

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report per-record and aggregate codon usage counts and frequencies

## Document Metadata

- Document ID: `chips-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `codon_tools`
- Legacy names: `chips`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/chips.validation.json`](../validation/chips.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`chips` reports deterministic codon-usage counts and frequencies for strict coding nucleotide sequences. It emits one normalized table that includes per-record codon counts as well as an aggregate profile across the full input set.

## Inputs

The current v1 interface accepts one local coding-sequence input. Records are read through the shared sequence loader and must be strict in-frame nucleotide coding sequences.

## Outputs

The result is a stable table report with per-record and aggregate codon rows. Each row records the scope, codon, amino acid, count, frequency, and derived weight field used by the shared codon-profile layer.

## Current Status

This method is implemented and exposed through `emboss-rs chips`. Rust service coverage exercises the committed reference coding fixture and checks that the aggregate table rows preserve the expected codon counts.

## Caveats

The v1 method is strict and local-only. Protein inputs are rejected. Internal stops, ambiguous codons, and non-triplet sequence lengths fail validation. A single terminal stop codon is allowed but excluded from the codon-profile counts.

## Declared Artifacts

### Codon-reference FASTA fixture

- Artifact ID: `codon_reference_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/codon_reference.fasta`
- Notes: Repository-managed coding-reference FASTA fixture used to validate per-record and aggregate codon-profile output.

## Declared Examples

### Report codon usage for a committed coding fixture

- Example ID: `report_codon_usage_for_reference_fixture`
- Description: Loads the committed coding-reference FASTA fixture and emits per-record plus aggregate codon-usage rows.
- Referenced artifacts: `codon_reference_fasta`
- Expected outputs:
  - `codon_usage_table`: Codon-usage table report (A stable table in which the aggregate profile includes a `CTT` count of `5` for the committed fixture set.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `report_codon_usage_for_reference_fixture`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes

