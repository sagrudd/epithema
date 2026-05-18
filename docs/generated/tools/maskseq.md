# maskseq

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Mask one or more explicit 1-based inclusive sequence intervals with molecule-aware default symbols

## Document Metadata

- Document ID: `maskseq-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `feature_tools`
- Legacy names: `maskseq`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/maskseq.validation.json`](../validation/maskseq.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`maskseq` masks explicit sequence intervals in each input record using the shared EMBOSS-RS interval and sequence-edit foundations. User-facing coordinates are 1-based inclusive, while the implementation converts them into the core zero-based half-open interval model before applying in-place masking through the shared `mask_intervals` helper.

## Inputs

The current v1 tool accepts a local sequence file followed by one or more `start:end` intervals. FASTA, FASTQ, EMBL, and GenBank inputs follow the shared sequence IO path. Multiple intervals are allowed and are applied deterministically in place to every input record.

## Outputs

The tool emits one masked sequence record per input record, preserving identifiers, descriptions, molecule kind, and other metadata. The default mask symbol is `N` for DNA, RNA, and currently unknown molecule kinds, and `X` for protein records. A custom mask character may be supplied when it is valid for the target record alphabet.

## Legacy Context

This acceptance anchor keeps one historical-style `maskseq` interval-masking example in view and compares the EMBOSS-RS FASTA payload against a committed expected output. The comparison validates the governed 1-based inclusive masking rule and default nucleotide mask symbol.

## Current Status

This method is implemented and exposed through `emboss-rs maskseq`. Validation currently covers deterministic masking of an interior interval against a committed three-record FASTA fixture. Rust tests also cover whole-sequence masking, protein default masking with `X`, and rejection of biologically invalid explicit mask symbols.

## Caveats

Intervals must be valid for every input record; EMBOSS-RS does not clip partial overlaps silently. Empty intervals are not representable because coordinates are 1-based inclusive with `start <= end`. Custom mask characters are conservative: they must be compatible with the record alphabet, so a protein mask cannot use `N` and a DNA mask cannot use protein-only symbols.

## Declared Artifacts

### Three-record FASTA fixture

- Artifact ID: `three_record_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/three_records.fasta`
- Notes: Repository-managed FASTA fixture used for deterministic maskseq validation.

## Declared Examples

### Mask positions 2 through 3 in each record

- Example ID: `mask_positions_two_to_three`
- Description: Applies the same 1-based inclusive interval to each record in a three-record FASTA fixture using the default nucleotide mask symbol.
- Referenced artifacts: `three_record_fasta`
- Parameters:
  - `interval` = `2:3`
- Expected outputs:
  - `masked_sequences`: Masked sequences (Each output record has positions 2 through 3 replaced with `N`, while identifiers and metadata are preserved.)
- Legacy reference: EMBOSS maskseq application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/maskseq.acd`
  - Invocation: `maskseq -sequence three_records.fasta -regions 2:3 -outseq stdout`

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS maskseq application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/maskseq.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `mask_positions_two_to_three`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes

