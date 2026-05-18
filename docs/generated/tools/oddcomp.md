# oddcomp

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report deterministic exact protein word-composition counts for query words

## Document Metadata

- Document ID: `oddcomp-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_stats`
- Legacy names: `oddcomp`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/oddcomp.validation.json`](../validation/oddcomp.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`oddcomp` identifies proteins by exact query-word composition. The EMBOSS-RS v1 implementation keeps the model deliberately simple and typed: one protein input, one or more exact query words, overlapping literal counting, and one stable table row per record per query word.

## Inputs

The current interface accepts one local protein input path plus one or more `--word` arguments. Inputs are loaded through the shared EMBOSS-RS readers for FASTA, FASTQ, EMBL, and GenBank. Nucleotide inputs are rejected.

## Outputs

The tool emits a stable table report with columns `record`, `query_word`, `word_length`, `count`, `frequency`, `contains`, and `counted_windows`. Rows preserve source record order and query-word order.

## Counting Model

Query words are normalized case-insensitively and validated as literal protein words. Counts are computed from overlapping exact matches within each record. Frequency is the count divided by the number of possible windows for that word length in the record.

## Current Status

This method is implemented and exposed through `emboss-rs oddcomp`. Validation currently covers exact overlapping protein-word counting, stable per-record reporting, and rejection of nucleotide inputs and invalid query words.

## Caveats

The first release does not implement broader statistical enrichment models, mismatch-tolerant motifs, or residue-class composition heuristics. It is an exact literal word-count surface only.

## Declared Artifacts

### Oddcomp protein fixture

- Artifact ID: `oddcomp_protein_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/oddcomp_records.fasta`
- Notes: Repository-managed protein fixture used to validate exact word-composition counting.

## Declared Examples

### Count exact protein query words

- Example ID: `count_exact_protein_words`
- Description: Counts overlapping exact `MAM` and `QQQ` words across a committed protein fixture and emits one stable row per record per query word.
- Referenced artifacts: `oddcomp_protein_fixture`
- Parameters:
  - `--word` = `MAM`
  - `--word` = `QQQ`
- Expected outputs:
  - `oddcomp_table`: Protein word-composition table (A stable table containing per-record counts and frequencies for each requested query word.)
- Legacy reference: EMBOSS oddcomp application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/oddcomp.acd`
  - Invocation: `oddcomp -sequence oddcomp_records.fasta -word MAM -word QQQ -stdout yes`

## Provenance

- Curated by: OpenAI Codex
- Source references:
  - EMBOSS oddcomp application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/oddcomp.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `count_exact_protein_words`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
