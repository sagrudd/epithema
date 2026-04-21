# notseq

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Exclude one sequence record by 1-based ordinal position

## Document Metadata

- Document ID: `notseq-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_stream`
- Legacy names: `notseq`

## Overview

`notseq` removes exactly one selected sequence record from a local sequence stream and returns the remaining records in their original order. The tool reuses the shared EMBOSS-RS sequence loader and sequence-collection result path instead of embedding format-specific stream logic.

## Inputs

The current v1 interface accepts one local sequence input path plus one exclusion index. The index is **1-based** and must refer to an existing record in the loaded input stream.

## Outputs

The tool emits the remaining sequence records through the shared FASTA output path. CLI output also includes the standard EMBOSS-RS method summary lines reporting the input path, excluded index, total input count, returned record count, and FASTA output format.

## Current Status

This method is implemented and exposed through `emboss-rs notseq`. Validation currently covers interior exclusion, first-record exclusion, single-record all-excluded behavior, and malformed or out-of-range failure cases.

## Caveats

The first release supports exclusion by ordinal position only. Identifier-based exclusion, multiple exclusion criteria, and duplicate-identifier policies remain deferred. Excluding the only record is allowed and returns an empty output stream.

## Declared Artifacts

### Three-record FASTA fixture

- Artifact ID: `three_record_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/three_records.fasta`
- Notes: Repository-managed multi-record FASTA fixture used to validate deterministic exclusion order.

## Declared Examples

### Exclude the second record from a three-record FASTA input

- Example ID: `exclude_second_record`
- Description: Removes the middle record from a small FASTA fixture and returns the remaining records in stable source order.
- Referenced artifacts: `three_record_fasta`
- Parameters:
  - `index` = `2`
- Expected outputs:
  - `filtered_sequence_collection`: Filtered sequence collection (A FASTA sequence collection containing the first and third records from the source fixture.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Validation Intent

- Required examples: `exclude_second_record`
- Compare against legacy: no
- Require provenance capture: yes

