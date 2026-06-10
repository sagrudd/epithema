# wordfinder

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

report maximal exact shared regions between one query and multiple targets

## Document Metadata

- Document ID: `wordfinder-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `pattern_tools`
- Legacy names: `wordfinder`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/wordfinder.validation.json`](../validation/wordfinder.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`wordfinder` extends the exact-region model of `wordmatch` to one query sequence against a target set. Epithema v1 preserves target-record order and reports maximal exact ungapped regions in a stable table.

## Inputs

The current interface accepts exactly one local query record, one local target set, and an optional `--word-size` threshold. Each target record must resolve to a compatible molecule class relative to the query.

## Outputs

The output is a stable table with one row per maximal exact shared region across the target set. Each row reports query and target identifiers, 1-based inclusive spans in both sequences, match length, and exact matched text.

## Current Status

This method is implemented and exposed through `epithema wordfinder`. Validation currently covers one committed query fixture and one committed two-record target set through the Rust tool and service layers.

## Caveats

The first release is an exact multi-target region reporter. It does not implement approximate neighborhoods, heuristic large-sequence acceleration, or graphical word-space output.

## Declared Artifacts

### Wordfinder query FASTA fixture

- Artifact ID: `wordfinder_query_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/wordmatch_query.fasta`
- Notes: Repository-managed one-record query FASTA fixture used for multi-target exact-region reporting.

### Wordfinder targets FASTA fixture

- Artifact ID: `wordfinder_targets_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/wordfinder_targets.fasta`
- Notes: Repository-managed two-record target FASTA fixture whose first record shares one exact region with the query.

## Declared Examples

### Report one exact shared region across a target set

- Example ID: `report_one_exact_shared_region_across_target_set`
- Description: Runs `wordfinder` against a committed one-record query FASTA fixture and a committed two-record target set.
- Referenced artifacts: `wordfinder_query_fasta`, `wordfinder_targets_fasta`
- Parameters:
  - `word_size` = `4`
- Expected outputs:
  - `wordfinder_hit_table`: Multi-target exact shared-region table (One row is reported for the target record `wf_target_a`, with no hit rows for the second target.)
- Legacy reference: EMBOSS wordfinder application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/wordfinder.acd`
  - Invocation: `wordfinder -asequence wordmatch_query.fasta -bsequence wordfinder_targets.fasta -wordsize 4 -outfile stdout`

## Provenance

- Curated by: epithema maintainers
- Source references:
  - EMBOSS wordfinder application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/wordfinder.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `report_one_exact_shared_region_across_target_set`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
