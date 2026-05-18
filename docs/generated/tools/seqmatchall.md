# seqmatchall

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

report all-against-all maximal exact shared regions across a sequence set

## Document Metadata

- Document ID: `seqmatchall-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `pattern_tools`
- Legacy names: `seqmatchall`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/seqmatchall.validation.json`](../validation/seqmatchall.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`seqmatchall` reports maximal exact ungapped shared regions for every record pair in one sequence set. EMBOSS-RS v1 compares each pair once in input order and emits every exact region meeting the minimum word size.

## Inputs

The current interface accepts one local sequence set plus an optional minimum word size. Molecule compatibility is enforced pairwise through the shared exact-word infrastructure.

## Outputs

The output is a deterministic table with left and right record identifiers, 1-based inclusive coordinates in both records, match length, and the exact shared region.

## Current Status

This method is implemented and exposed through `emboss-rs seqmatchall`. Validation currently covers one committed three-record FASTA fixture whose three pairings each yield the same maximal 4-residue exact region.

## Caveats

The first release reports exact ungapped regions only. It does not perform heuristic seeding, gapped extension, or EMBOSS-era scoring beyond maximal exact region extraction.

## Declared Artifacts

### Seqmatchall multi-record FASTA fixture

- Artifact ID: `seqmatchall_records_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/seqmatchall_records.fasta`
- Notes: Repository-managed three-record FASTA fixture used for deterministic all-against-all exact-region reporting.

## Declared Examples

### Report all-against-all maximal exact regions

- Example ID: `report_all_against_all_exact_regions`
- Description: Runs `seqmatchall` against a committed three-record FASTA fixture with a minimum word size of 4.
- Referenced artifacts: `seqmatchall_records_fasta`
- Parameters:
  - `word_size` = `4`
- Expected outputs:
  - `seqmatchall_hit_table`: All-against-all exact shared-region table (Three pairwise rows are reported, each carrying the shared exact region `ACGT` with stable pair ordering.)
- Legacy reference: EMBOSS seqmatchall application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/seqmatchall.acd`
  - Invocation: `seqmatchall -sequence seqmatchall_records.fasta -wordsize 4 -outfile stdout`

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS seqmatchall application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/seqmatchall.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `report_all_against_all_exact_regions`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes

