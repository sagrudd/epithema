# seqret

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Normalize local sequence inputs or retrieve one provider-backed sequence accession

## Document Metadata

- Document ID: `seqret-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `retrieval_tools`
- Legacy names: `seqret`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/seqret.validation.json`](../validation/seqret.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`seqret` is the current EMBOSS-RS sequence retrieval and normalization entry point. In v1 it supports two real paths: reading a local sequence file through the shared sequence IO layer, or retrieving one provider-qualified accession through the governed provider seam and returning the result as normalized sequence records.

## Inputs

The current interface accepts exactly one governed input reference. Local sequence files are supported. Provider-backed accessions are supported when they are explicitly qualified, for example `ena:AB000263`. Bare accessions that could resolve through more than one provider are rejected conservatively, and inline literal sequence text is not supported in the `seqret` service path.

## Outputs

The result is a normalized sequence collection rendered through the shared sequence output path. Local multi-record inputs preserve input record order. Provider-backed retrieval currently returns the fetched record with provenance attached to the core sequence metadata.

## Current Status

This method is implemented and exposed through `emboss-rs seqret`. The local-file normalization path is exercised directly against a committed FASTA fixture. The provider-backed accession path is also implemented and covered in Rust service tests using mocked remote retrieval so the retrieval seam is executable without claiming that live network acceptance evidence has been harvested yet.

## Caveats

The first release is intentionally narrow. `seqret` is not yet a broad accession-inference or free-text input tool. Ambiguous bare accessions are rejected, inline sequence literals are unsupported in the service layer, and the current curated evidence proves the governed seam and normalization behavior rather than historical EMBOSS parity across all retrieval modes.

## Declared Artifacts

### Three-record FASTA fixture

- Artifact ID: `three_record_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/three_records.fasta`
- Notes: Repository-managed multi-record FASTA fixture used to validate local normalization and preserved record ordering.

## Declared Examples

### Normalize a local multi-record FASTA input

- Example ID: `normalize_local_fasta_records`
- Description: Loads the committed three-record FASTA fixture through the shared sequence reader and returns the same three records as a normalized sequence collection.
- Referenced artifacts: `three_record_fasta`
- Expected outputs:
  - `normalized_local_sequence_collection`: Normalized local sequence collection (A three-record output collection that preserves the source order and stable record identifiers.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `normalize_local_fasta_records`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes

