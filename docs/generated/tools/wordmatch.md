# wordmatch

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

report maximal exact shared regions between two singleton sequences

## Document Metadata

- Document ID: `wordmatch-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `pattern_tools`
- Legacy names: `wordmatch`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/wordmatch.validation.json`](../validation/wordmatch.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`wordmatch` finds maximal exact ungapped shared regions between exactly one query sequence and one target sequence. EMBOSS-RS v1 reports a stable table of exact identity regions instead of producing a dotplot or an exploratory visual summary.

## Inputs

The current interface accepts exactly one local sequence record from each of two inputs plus an optional `--word-size` threshold. Both records must resolve to a compatible molecule class.

## Outputs

The output is a stable table with one row per maximal exact shared region that satisfies the minimum word-size threshold. Each row reports the query and target identifiers, 1-based inclusive spans in both sequences, the match length, and the matched exact text.

## Current Status

This method is implemented and exposed through `emboss-rs wordmatch`. Validation currently covers one committed exact-region pair through the Rust tool and service layers.

## Caveats

The first release reports exact ungapped regions only. It does not compute wordmatch dotplots, chain approximate neighborhoods, or add scoring beyond the minimum word-size threshold.

## Declared Artifacts

### Wordmatch query FASTA fixture

- Artifact ID: `wordmatch_query_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/wordmatch_query.fasta`
- Notes: Repository-managed one-record query FASTA fixture used for exact shared-region reporting.

### Wordmatch target FASTA fixture

- Artifact ID: `wordmatch_target_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/wordmatch_target.fasta`
- Notes: Repository-managed one-record target FASTA fixture sharing one exact region with the query.

## Declared Examples

### Report one exact shared region

- Example ID: `report_one_exact_shared_region`
- Description: Runs `wordmatch` against two committed one-record FASTA fixtures that share one maximal exact region of length four.
- Referenced artifacts: `wordmatch_query_fasta`, `wordmatch_target_fasta`
- Parameters:
  - `word_size` = `4`
- Expected outputs:
  - `wordmatch_hit_table`: Exact shared-region table (One row is reported for the maximal exact region `ACGT`.)
- Legacy reference: EMBOSS wordmatch application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/wordmatch.acd`
  - Invocation: `wordmatch -sequence wordmatch_query.fasta -asequence wordmatch_target.fasta -wordsize 4 -outfile stdout`

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS wordmatch application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/wordmatch.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `report_one_exact_shared_region`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes

