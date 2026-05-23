# matcher

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Deterministic ungapped pairwise similarity summary

## Document Metadata

- Document ID: `matcher-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `alignment_analysis`
- Legacy names: `matcher`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/matcher.validation.json`](../validation/matcher.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`matcher` currently provides a conservative direct-comparison summary between exactly one query record and one target record. EMBOSS-RS v1 compares the shared positional overlap without introducing gaps and reports identities, mismatches, compared length, identity percentage, and any input-length difference in a stable table form.

## Inputs

The current v1 interface accepts exactly one local query sequence input and one local target sequence input. Each input must resolve to exactly one sequence record through the shared sequence loader.

## Outputs

The output is a deterministic single-row summary table rather than a rendered local alignment. The table reports the query and target identifiers, compared length, input lengths, identity count, mismatch count, and overlap identity percentage.

## Current Status

This method is implemented and exposed through `emboss-rs matcher`. Current Rust service coverage exercises committed singleton FASTA fixtures and locks down the expected direct-comparison counts for the overlap between `ACGT` and `ACT`.

## Caveats

The historical EMBOSS name suggests richer local-alignment behavior, but the current EMBOSS-RS v1 implementation is deliberately narrower. It does not perform Waterman-Eggert local alignment yet; it performs ungapped positional comparison only.

## Declared Artifacts

### Matcher query FASTA fixture

- Artifact ID: `matcher_query_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/needle_query.fasta`
- Notes: Repository-managed singleton FASTA fixture used as the query input.

### Matcher target FASTA fixture

- Artifact ID: `matcher_target_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/needle_target.fasta`
- Notes: Repository-managed singleton FASTA fixture used as the target input.

## Declared Examples

### Compare two singleton FASTA records over their shared overlap

- Example ID: `compare_singleton_sequences_without_gaps`
- Description: Compares the committed `ACGT` query fixture against the `ACT` target fixture and reports the deterministic ungapped overlap statistics.
- Referenced artifacts: `matcher_query_fasta`, `matcher_target_fasta`
- Expected outputs:
  - `matcher_summary_table`: Direct-comparison summary table (A one-row table reporting compared length, identities, mismatches, and overlap identity percentage.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `compare_singleton_sequences_without_gaps`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
