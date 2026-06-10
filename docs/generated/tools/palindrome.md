# palindrome

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

report exact reverse-complement palindromic regions in nucleotide sequences

## Document Metadata

- Document ID: `palindrome-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `pattern_tools`
- Legacy names: `palindrome`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/palindrome.validation.json`](../validation/palindrome.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`palindrome` reports exact reverse-complement palindromic windows in nucleotide sequence records. Epithema v1 searches the forward source sequence only and emits every exact palindrome within the configured inclusive length range.

## Inputs

The current interface accepts one local nucleotide input plus optional minimum and maximum palindrome lengths. Protein and unknown-classified inputs are rejected because reverse-complement semantics are required.

## Outputs

The result is a stable table with record identifier, 1-based inclusive start and end coordinates, window length, and the matched palindromic sequence.

## Current Status

This method is implemented and exposed through `epithema palindrome`. Validation currently covers exact 6-residue palindrome reporting against a committed nucleotide FASTA fixture through the Rust tool and service layers.

## Caveats

The first release does not collapse nested palindromes, does not score imperfect palindromes, and does not search reverse-strand coordinates separately from the forward source sequence.

## Declared Artifacts

### Palindrome nucleotide FASTA fixture

- Artifact ID: `palindrome_records_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/palindrome_records.fasta`
- Notes: Repository-managed nucleotide FASTA fixture containing exact 6-residue palindromic windows.

## Declared Examples

### Report exact 6-residue palindromic windows

- Example ID: `report_bounded_exact_palindromes`
- Description: Runs `palindrome` against a committed nucleotide FASTA fixture with both minimum and maximum length set to 6.
- Referenced artifacts: `palindrome_records_fasta`
- Parameters:
  - `min_length` = `6`
  - `max_length` = `6`
- Expected outputs:
  - `palindrome_hit_table`: Exact palindrome hit table (Two exact 6-residue palindrome rows are reported with stable 1-based inclusive coordinates.)
- Legacy reference: EMBOSS palindrome application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/palindrome.acd`
  - Invocation: `palindrome -sequence palindrome_records.fasta -minpallen 6 -maxpallen 6 -outfile stdout`

## Provenance

- Curated by: epithema maintainers
- Source references:
  - EMBOSS palindrome application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/palindrome.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `report_bounded_exact_palindromes`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
