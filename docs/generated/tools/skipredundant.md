# skipredundant

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Remove exact duplicate sequence records while preserving first-seen representatives

## Document Metadata

- Document ID: `skipredundant-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_stream`
- Legacy names: `skipredundant`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/skipredundant.validation.json`](../validation/skipredundant.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`skipredundant` collapses an input sequence stream to exact non-redundant representatives. Epithema v1 defines redundancy conservatively as identical molecule kind plus identical normalized residue content, and it retains the first representative encountered for each exact sequence.

## Inputs

The current interface accepts one local sequence input path containing one or more sequence records. All records are loaded through the shared Epithema sequence IO layer before redundancy filtering is applied.

## Outputs

The tool emits a non-redundant FASTA sequence collection through the shared result path. CLI summaries report the input path, total loaded record count, exact duplicates removed, and returned representative count.

## Current Status

This method is implemented and exposed through `epithema skipredundant`. Validation currently covers deterministic removal of exact duplicate records against a committed FASTA fixture, including service-level summary checks.

## Caveats

The first release does not support historical similarity-threshold redundancy, alignment-based clustering, or accession-aware deduplication. Duplicate handling is exact and sequence-based only.

## Declared Artifacts

### Exact-duplicate FASTA fixture

- Artifact ID: `skipredundant_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/skipredundant_records.fasta`
- Notes: Repository-managed FASTA fixture containing two exact duplicate pairs for deterministic first-representative retention.

## Declared Examples

### Keep only the first representative of each exact sequence

- Example ID: `remove_exact_duplicate_records`
- Description: Runs `skipredundant` against a committed FASTA fixture containing exact duplicates and emits only the first representative for each sequence content key.
- Referenced artifacts: `skipredundant_fixture`
- Expected outputs:
  - `non_redundant_sequence_collection`: Non-redundant FASTA collection (A FASTA sequence collection containing only `keep_alpha` and `keep_gamma`, with two duplicates removed.)

## Provenance

- Curated by: epithema maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `remove_exact_duplicate_records`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
