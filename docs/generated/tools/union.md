# union

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Combine two or more sequence inputs into one deterministic output stream

## Document Metadata

- Document ID: `union-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_transform`
- Legacy names: `union`

## Overview

`union` combines two or more local sequence inputs into one output stream by stable concatenation. The first EMBOSS-RS release keeps the semantics intentionally simple: preserve input order, preserve per-input record order, and preserve duplicates exactly as they were read.

## Inputs

The current v1 interface requires at least two local sequence input paths. Each input is loaded through the shared EMBOSS-RS sequence readers for FASTA, FASTQ, EMBL, and GenBank.

## Outputs

The tool emits one combined FASTA sequence collection through the shared output path. CLI output also includes the standard EMBOSS-RS method summary lines reporting input count, output record count, ordering policy, duplicate policy, and FASTA output format.

## Current Status

This method is implemented and exposed through `emboss-rs union`. Validation currently covers two-input concatenation, duplicate-identifier preservation, too-few-input failure, and malformed or empty-input failure behavior.

## Caveats

The first release does not perform identifier-based or content-based deduplication. Records that share identifiers, descriptions, or sequence content are preserved as independent output records in first-seen order.

## Declared Artifacts

### Three-record FASTA fixture

- Artifact ID: `three_record_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/three_records.fasta`
- Notes: Repository-managed FASTA fixture used as the leading input for deterministic union validation.

### Two-record FASTA fixture

- Artifact ID: `two_record_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/two_records.fasta`
- Notes: Repository-managed FASTA fixture appended after the three-record input during union validation.

## Declared Examples

### Concatenate two FASTA inputs in stable order

- Example ID: `concatenate_two_sequence_inputs`
- Description: Appends a two-record FASTA fixture after a three-record FASTA fixture and returns the five-record combined stream in deterministic order.
- Referenced artifacts: `three_record_fasta`, `two_record_fasta`
- Expected outputs:
  - `union_sequence_collection`: Combined sequence collection (A FASTA sequence collection containing the first input records followed by the second input records without deduplication.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Validation Intent

- Required examples: `concatenate_two_sequence_inputs`
- Compare against legacy: no
- Require provenance capture: yes

