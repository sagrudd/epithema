# seqretsetall

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Normalize multiple local or provider-backed sequence inputs into ordered output record sets

## Document Metadata

- Document ID: `seqretsetall-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `retrieval_tools`
- Legacy names: `seqretsetall`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/seqretsetall.validation.json`](../validation/seqretsetall.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`seqretsetall` is the current bounded many-set retrieval companion to `seqret`. In v1 it resolves two or more governed sequence inputs through the same local/provider-aware substrate as `seqret` and returns deterministic ordered partitions of normalized sequence records.

## Inputs

The current interface accepts at least two governed input references. Local sequence files are supported. Provider-backed accessions are supported when they are explicitly qualified, for example `ena:AB000263`. Empty resolved input sets are rejected conservatively, and inline literal sequence text is not supported in the `seqretsetall` service path.

## Outputs

The result is a partitioned normalized sequence collection rendered through the governed sequence-partition surface. Each resolved input contributes one ordered output record set, preserving both input ordering and per-input record ordering.

## Legacy Context

This acceptance anchor keeps one bounded historical-style `seqretsetall` normalization example in view and compares the Epithema partitioned output against a committed expected payload. The comparison is deliberately narrow and local-fixture based; it does not claim broad provider or filesystem-policy parity.

## Current Status

This method is implemented and exposed through `epithema seqretsetall`. The bounded local many-set path is compared against a committed partition fixture through the acceptance-anchor harness, and Rust service coverage also exercises a mixed local plus mocked-provider path through the explicit provider seam. That proves the governed retrieval surface without claiming harvested live-provider acceptance evidence yet.

## Caveats

The first release is intentionally narrow. `seqretsetall` is not yet a generic batching or filesystem-policy framework, does not accept inline sequence literals, and does not claim broad provider-parity beyond the governed explicit-provider seam already exercised in Rust tests.

## Declared Artifacts

### Three-record FASTA fixture

- Artifact ID: `three_record_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/three_records.fasta`
- Notes: Repository-managed multi-record FASTA fixture used as the first ordered input set in the bounded local many-set retrieval example.

### Two-record FASTA fixture

- Artifact ID: `two_record_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/two_records.fasta`
- Notes: Repository-managed two-record FASTA fixture used as the second ordered input set in the bounded local many-set retrieval example.

### Mocked mixed many-set retrieval case

- Artifact ID: `seqretsetall_mocked_mixed_case`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-testkit/tests/fixtures/autodoc/seqretsetall_mixed_case.md`
- Notes: Repository-managed case note for the bounded mixed local plus provider-qualified many-set retrieval path used in Rust service coverage.

## Declared Examples

### Normalize multiple sequence inputs into ordered output partitions

- Example ID: `normalize_multiple_sequence_inputs_into_ordered_partitions`
- Description: Resolves two committed local FASTA inputs through the shared retrieval substrate and returns one ordered normalized output record set per input.
- Referenced artifacts: `three_record_fasta`, `two_record_fasta`
- Expected outputs:
  - `partitioned_sequence_collection`: Partitioned normalized sequence collection (A sequence-partition payload that preserves input-set ordering and per-input record ordering across local and explicitly provider-qualified sources.)
- Legacy reference: EMBOSS seqretsetall application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/seqretsetall.acd`
  - Invocation: `seqretsetall -sequence three_records.fasta two_records.fasta -outseq stdout`

## Provenance

- Curated by: epithema maintainers
- Source references:
  - EMBOSS seqretsetall application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/seqretsetall.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `normalize_multiple_sequence_inputs_into_ordered_partitions`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes
