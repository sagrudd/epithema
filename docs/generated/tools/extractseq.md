# extractseq

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Extract one contiguous 1-based inclusive region from each input sequence record

## Document Metadata

- Document ID: `extractseq-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_transform`
- Legacy names: `extractseq`

## Overview

`extractseq` extracts the same contiguous region from every input sequence record using the shared EMBOSS-RS interval model. User-facing coordinates are 1-based inclusive, while the implementation converts them into the core zero-based half-open interval type before slicing sequence content.

## Inputs

The current v1 tool accepts a local sequence file plus `start` and `end` coordinates. FASTA, FASTQ, EMBL, and GenBank inputs follow the shared sequence IO layer. The requested interval must be valid for every input record in the file.

## Outputs

The tool emits one extracted sequence record per input record, preserving input order, identifiers, descriptions, and molecule kind. Extraction is generic across nucleotide and protein records because it operates on the shared `SequenceRecord` abstraction.

## Current Status

This method is implemented and exposed through `emboss-rs extractseq`. Validation currently covers interior extraction against a committed multi-record FASTA fixture. Boundary cases, full-length extraction, and invalid coordinate handling are covered by Rust tests in the tool and service layers.

## Caveats

The v1 scope extracts a single contiguous interval only. Coordinates must be supplied as 1-based inclusive values with `start <= end`. Empty ranges are therefore not representable. If any record is shorter than the requested region, the run fails clearly instead of truncating silently.

## Declared Artifacts

### Three-record FASTA fixture

- Artifact ID: `three_record_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/three_records.fasta`
- Notes: Repository-managed FASTA fixture used for deterministic extractseq validation.

## Declared Examples

### Extract positions 2 through 3 from each record

- Example ID: `extract_region_two_to_three`
- Description: Applies the same 1-based inclusive interval to each record in a multi-record FASTA fixture.
- Referenced artifacts: `three_record_fasta`
- Parameters:
  - `start` = `2`
  - `end` = `3`
- Expected outputs:
  - `extracted_sequences`: Extracted subsequences (Each output record contains positions 2 through 3 from the corresponding input record.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Validation Intent

- Required examples: `extract_region_two_to_three`
- Compare against legacy: no
- Require provenance capture: yes

