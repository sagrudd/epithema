# shuffleseq

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Shuffle sequence residues deterministically while preserving composition

## Document Metadata

- Document ID: `shuffleseq-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_transform`
- Legacy names: `shuffleseq`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/shuffleseq.validation.json`](../validation/shuffleseq.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`shuffleseq` permutes the residues within each input record while preserving exact per-record composition. The Epithema v1 implementation is deterministic by design: when `--seed` is omitted it uses a fixed documented base seed, and each record derives its own deterministic stream from that base seed and input order.

## Inputs

The current interface accepts one local sequence input plus an optional `--seed`. Multi-record inputs are supported. Record order is preserved, and each record is shuffled independently rather than pooling residues across the full input stream.

## Outputs

The tool emits one shuffled FASTA sequence collection. CLI summaries report the source input, the base seed used, the deterministic per-record permutation rule, exact composition preservation, and FASTA output format.

## Current Status

This method is implemented and exposed through `epithema shuffleseq`. Validation currently covers deterministic seeded shuffling and composition preservation against a committed multi-record FASTA fixture, and Rust service tests exercise the same repository-managed case.

## Caveats

The first release does not offer cryptographic randomness, cross-record shuffling, or feature-coordinate preservation. It preserves identifiers and metadata, but any attached feature coordinates should not be treated as biologically meaningful after shuffling.

## Declared Artifacts

### Three-record FASTA fixture

- Artifact ID: `three_record_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/three_records.fasta`
- Notes: Repository-managed three-record FASTA fixture used for deterministic shuffleseq validation.

## Declared Examples

### Shuffle a three-record FASTA fixture with an explicit seed

- Example ID: `shuffle_records_with_seed_7`
- Description: Runs `shuffleseq` against the committed three-record FASTA fixture with `--seed 7` and returns deterministic per-record permutations that preserve composition exactly.
- Referenced artifacts: `three_record_fasta`
- Parameters:
  - `--seed` = `7`
- Expected outputs:
  - `shuffled_sequence_collection`: Deterministically shuffled sequence collection (A FASTA sequence collection whose first record residues are `CGTA`, while overall per-record composition remains unchanged.)

## Provenance

- Curated by: epithema maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `shuffle_records_with_seed_7`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
