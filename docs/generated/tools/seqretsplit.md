# seqretsplit

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Normalize one local or provider-backed sequence input into deterministic per-record split-output partitions

## Document Metadata

- Document ID: `seqretsplit-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `retrieval_tools`
- Legacy names: `seqretsplit`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/seqretsplit.validation.json`](../validation/seqretsplit.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`seqretsplit` is the current bounded split-output retrieval companion to `seqret` and `seqretsetall`. In v1 it resolves one governed sequence input through the same local/provider-aware substrate and returns deterministic one-record partitions that correspond to explicit output file names.

## Inputs

The current interface accepts exactly one governed input reference. Local sequence files are supported. Provider-backed accessions are supported when they are explicitly qualified, for example `ena:AB000263`. Empty resolved record sets are rejected conservatively, and inline literal sequence text is not supported in the `seqretsplit` service path.

## Outputs

The result is a partitioned normalized sequence collection rendered through the governed sequence-partition surface, with exactly one normalized record per partition. The same computation path also derives deterministic output file names for each partition so the split-output policy stays explicit without broad filesystem orchestration claims.

## Legacy Context

This bounded release keeps the historical `seqretsplit` user need in scope while modernizing around deterministic normalized record partitions instead of raw legacy file-writing behavior. The first shipped slice proves the governed split-output surface without claiming broad filename-policy or provider-parity equivalence.

## Current Status

This method is implemented and exposed through `emboss-rs seqretsplit`. The bounded local path and the explicit-provider mocked path both execute through Rust service coverage, but this task boundary stops at shipped executable evidence only; canonical compared fixtures have not landed yet.

## Caveats

The first release is intentionally narrow. `seqretsplit` is not yet a generic directory-policy or filename-policy framework, does not accept inline sequence literals, and does not claim broad provider-parity beyond the governed explicit-provider seam already exercised in Rust tests.

## Declared Artifacts

### Three-record FASTA fixture

- Artifact ID: `three_record_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/three_records.fasta`
- Notes: Repository-managed three-record FASTA fixture used for the bounded local split-output example.

### Mocked provider split-output case

- Artifact ID: `seqretsplit_mocked_provider_case`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-testkit/tests/fixtures/autodoc/seqretsplit_mixed_case.md`
- Notes: Repository-managed case note for the bounded provider-qualified split-output path exercised through the Rust service seam.

## Declared Examples

### Normalize one sequence input into deterministic split partitions

- Example ID: `normalize_one_sequence_input_into_deterministic_split_partitions`
- Description: Resolves one committed local FASTA input through the shared retrieval substrate and returns one normalized output partition per record with an explicit deterministic file name.
- Referenced artifacts: `three_record_fasta`
- Expected outputs:
  - `split_partitioned_sequence_collection`: Split-output normalized sequence collection (A sequence-partition payload with one normalized record per partition, aligned with deterministic split-output file names from the same computation path.)
- Legacy reference: EMBOSS seqretsplit application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/seqretsplit.acd`
  - Invocation: `seqretsplit -sequence three_records.fasta -outseq stdout`

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS seqretsplit application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/seqretsplit.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `normalize_one_sequence_input_into_deterministic_split_partitions`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes
