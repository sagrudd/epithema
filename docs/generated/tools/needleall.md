# needleall

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Perform deterministic many-vs-many global pairwise alignment and report comparison summaries

## Document Metadata

- Document ID: `needleall-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `pairwise_alignment`
- Legacy names: `needleall`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/needleall.validation.json`](../validation/needleall.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`needleall` performs deterministic many-vs-many global pairwise alignment between every query record and every target record in two local sequence sets. Epithema v1 keeps the surface deliberately structured: comparisons run in query-major then target-major order and the CLI reports one summary row per pair instead of rendering every alignment body.

## Inputs

The current v1 interface accepts one local query sequence-set input and one local target sequence-set input, plus optional `--gap-open` and `--gap-extend` overrides. Both inputs must contain at least one sequence record.

## Outputs

CLI and service output is a deterministic summary table with one row per query/target pair. Each row reports the query identifier, target identifier, scoring mode, alignment score, aligned length, identity count, identity percentage, and query/target gap counts.

## Current Status

This method is implemented and exposed through `epithema needleall`. Current Rust service coverage exercises committed multi-record FASTA fixtures and locks down the query-major, target-major ordering of the four pairwise comparisons.

## Caveats

The first release reports a structured comparison table only. It does not emit every rendered pairwise alignment body or reproduce the full breadth of historical EMBOSS reporting options yet.

## Declared Artifacts

### Needleall query FASTA fixture

- Artifact ID: `needleall_queries_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/needleall_queries.fasta`
- Notes: Repository-managed two-record FASTA fixture used as the query set.

### Needleall target FASTA fixture

- Artifact ID: `needleall_targets_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/needleall_targets.fasta`
- Notes: Repository-managed two-record FASTA fixture used as the target set.

## Declared Examples

### Align every query/target pair from two small FASTA sets

- Example ID: `align_all_query_target_pairs`
- Description: Runs deterministic many-vs-many global alignment across the committed two-record query and two-record target fixtures, yielding four summary rows in query-major then target-major order.
- Referenced artifacts: `needleall_queries_fasta`, `needleall_targets_fasta`
- Expected outputs:
  - `needleall_summary_table`: Many-vs-many alignment summary table (A four-row summary table covering every query/target pair in deterministic order.)

## Provenance

- Curated by: epithema maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `align_all_query_target_pairs`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
