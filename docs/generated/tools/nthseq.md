# nthseq

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Select one sequence record by 1-based ordinal position

## Document Metadata

- Document ID: `nthseq-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_stream`
- Legacy names: `nthseq`

## Overview

`nthseq` selects exactly one sequence record from a local sequence stream by ordinal position. The tool reuses the shared EMBOSS-RS sequence loader and typed single-record result path instead of re-parsing format-specific content inside the command.

## Inputs

The current v1 interface accepts one local sequence input path plus one selection index. The index is **1-based** and must refer to an existing record in the loaded input stream.

## Outputs

The tool emits one selected sequence record through the shared FASTA output path. CLI output also includes the standard EMBOSS-RS method summary lines reporting the input path, selected index, total record count, and FASTA output format.

## Current Status

This method is implemented and exposed through `emboss-rs nthseq`. Validation currently covers first, interior, and last-record selection plus empty-input, malformed-input, and out-of-range failure behavior.

## Caveats

The first release supports ordinal selection only. Identifier-based selection, multiple returned records, and grouped reporting remain deferred. Duplicate identifiers do not affect behavior because selection is position-based.

## Declared Artifacts

### Three-record FASTA fixture

- Artifact ID: `three_record_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/three_records.fasta`
- Notes: Repository-managed multi-record FASTA fixture used to validate stable ordinal selection.

## Declared Examples

### Select the second record from a three-record FASTA input

- Example ID: `select_second_record`
- Description: Selects the middle record from a small FASTA fixture and returns only that sequence record.
- Referenced artifacts: `three_record_fasta`
- Parameters:
  - `index` = `2`
- Expected outputs:
  - `selected_sequence_record`: Selected sequence record (A FASTA sequence record containing the second record from the source fixture.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Validation Intent

- Required examples: `select_second_record`
- Compare against legacy: no
- Require provenance capture: yes

