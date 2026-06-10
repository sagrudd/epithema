# cai

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report deterministic codon adaptation index values against a reference profile

## Document Metadata

- Document ID: `cai-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `codon_tools`
- Legacy names: `cai`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/cai.validation.json`](../validation/cai.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`cai` computes deterministic CAI-like values for strict coding nucleotide sequences. It derives a reference codon-usage profile from either another coding-sequence input or a normalized profile emitted by `codcopy`, then scores each query record against that reference.

## Inputs

The current v1 interface accepts two local inputs: a coding query input and a reference input. The query input is read through the shared sequence loader and must contain strict in-frame nucleotide coding sequences. The reference input may be another coding-sequence file or a normalized codon-profile TSV produced by `codcopy`.

## Outputs

The result is a stable table report with one row per query record. It includes the record identifier, sense-codon count, optional terminal stop codon, and the computed CAI-like value. Stop codons are excluded from weight derivation and a zero-weight codon in the reference yields CAI `0.0` for the affected query.

## Current Status

This method is implemented and exposed through `epithema cai`. Rust service coverage exercises CAI scoring directly against committed coding fixtures, and `codcopy` interoperability is also tested by writing a temporary normalized profile and reusing it as the CAI reference input.

## Caveats

The v1 method is intentionally strict. Protein inputs are rejected. Query sequences with non-triplet lengths, ambiguous codons, or internal stop codons fail validation. A single terminal stop codon is allowed and reported separately but excluded from the codon profile.

## Declared Artifacts

### Codon-query FASTA fixture

- Artifact ID: `codon_query_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/codon_query.fasta`
- Notes: Repository-managed coding-query FASTA fixture with one preferred-codon example and one rare-codon example.

### Codon-reference FASTA fixture

- Artifact ID: `codon_reference_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/codon_reference.fasta`
- Notes: Repository-managed coding-reference FASTA fixture used to derive deterministic synonymous codon weights.

## Declared Examples

### Score coding query sequences against a coding reference fixture

- Example ID: `score_query_sequences_against_reference_fixture`
- Description: Derives a codon-usage reference from the committed coding-reference fixture and reports CAI-like values for the committed query records.
- Referenced artifacts: `codon_query_fasta`, `codon_reference_fasta`
- Expected outputs:
  - `cai_table_rows`: CAI result rows (A two-row table in which the preferred-codon query scores higher than the rare-codon query.)

## Provenance

- Curated by: epithema maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `score_query_sequences_against_reference_fixture`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
