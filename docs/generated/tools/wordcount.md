# wordcount

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Count overlapping normalized sequence words with stable per-record and aggregate reporting

## Document Metadata

- Document ID: `wordcount-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_stats`
- Legacy names: `wordcount`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/wordcount.validation.json`](../validation/wordcount.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`wordcount` counts overlapping normalized sequence words across one or more input records. The EMBOSS-RS v1 surface deliberately keeps the model exact and deterministic: words are raw normalized substrings of a fixed width, windows overlap, and counts are reported both per record and in one aggregate summary.

## Inputs

The current interface accepts one local sequence input path plus a required word size. Inputs are loaded through the shared EMBOSS-RS readers for FASTA, FASTQ, EMBL, and GenBank and may contain nucleotide, protein, or mixed record sets.

## Outputs

The tool emits a stable table report with columns `scope`, `record`, `molecule`, `word_size`, `word`, `count`, `frequency`, and `skipped_gap_windows`. `scope` is either `record` or `aggregate`. Aggregate rows use `record=ALL` and `molecule=mixed`.

## Counting Model

Words are counted from overlapping windows of length `word_size` in the normalized residue string. Windows containing `-` are skipped and tracked through `skipped_gap_windows`. All other symbols, including ambiguity codes or stop symbols, are treated literally. Frequencies are computed over counted windows after gap-window exclusion.

## Current Status

This method is implemented and exposed through `emboss-rs wordcount`. Validation currently covers overlapping word counts, aggregate counting across multiple records, skipped gap-window handling, and stable service-level table emission.

## Caveats

The first release does not implement richer residue-class or mismatch-aware word models. It also does not emit a separate extracted-word sequence set; it reports counts only.

## Declared Artifacts

### Three-record FASTA fixture

- Artifact ID: `three_record_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/three_records.fasta`
- Notes: Repository-managed multi-record FASTA fixture used to validate deterministic overlapping word counts.

## Declared Examples

### Count overlapping sequence words

- Example ID: `count_overlapping_words`
- Description: Counts overlapping size-2 words across a committed three-record FASTA fixture and emits per-record plus aggregate rows.
- Referenced artifacts: `three_record_fasta`
- Parameters:
  - `--word-size` = `2`
- Expected outputs:
  - `wordcount_table`: Sequence word-count table (A stable tabular report with overlapping word counts and frequencies after gap-window exclusion.)
- Legacy reference: EMBOSS wordcount application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/wordcount.acd`
  - Invocation: `wordcount -sequence three_records.fasta -wordsize 2 -outfile stdout`

## Provenance

- Curated by: OpenAI Codex
- Source references:
  - EMBOSS wordcount application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/wordcount.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `count_overlapping_words`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes

