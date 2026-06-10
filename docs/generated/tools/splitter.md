# splitter

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Partition an input sequence stream into deterministic fixed-size chunks

## Document Metadata

- Document ID: `splitter-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_transform`
- Legacy names: `splitter`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/splitter.validation.json`](../validation/splitter.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`splitter` partitions an input record stream into fixed-size groups while preserving input order. The Epithema v1 implementation models this as deterministic sequence partitions rather than as side-effectful file fan-out, which keeps the method reusable through the shared service and bridge layers.

## Inputs

The current interface accepts one local sequence input and one positive `chunk-size`. Multi-record sequence inputs are supported through the shared sequence IO path. A chunk size of `1` produces one record per partition, while the final partition may be smaller than the requested chunk size.

## Outputs

The service payload is a deterministic partitioned sequence result in which outer partition ordering and inner record ordering both follow the original input stream. CLI rendering remains sequence-oriented rather than creating multiple output files in v1.

## Current Status

This method is implemented and exposed through `epithema splitter`. Validation currently covers deterministic partition sizes against a committed three-record FASTA fixture, and Rust service tests exercise the same repository-managed case.

## Caveats

The first release does not create numbered output files or support multiple simultaneous chunking policies. It reports logical partitions only, and it rejects chunk sizes smaller than `1`.

## Declared Artifacts

### Three-record FASTA fixture

- Artifact ID: `three_record_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/three_records.fasta`
- Notes: Repository-managed FASTA fixture used for deterministic splitter validation.

## Declared Examples

### Partition a three-record FASTA stream into chunks of two records

- Example ID: `split_three_records_into_two_partitions`
- Description: Runs `splitter` against the committed three-record FASTA fixture with chunk size `2`, producing one two-record partition and one final single-record partition.
- Referenced artifacts: `three_record_fasta`
- Parameters:
  - `chunk-size` = `2`
- Expected outputs:
  - `sequence_partitions`: Deterministic sequence partitions (A stable partition result containing records `alpha` and `beta` in the first chunk and `gamma` in the second chunk.)

## Provenance

- Curated by: epithema maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `split_three_records_into_two_partitions`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
