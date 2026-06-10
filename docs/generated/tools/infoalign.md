# infoalign

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report row counts, column counts, and per-row gap statistics for an alignment

## Document Metadata

- Document ID: `infoalign-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `alignment_tools`
- Legacy names: `infoalign`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/infoalign.validation.json`](../validation/infoalign.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`infoalign` reports stable alignment metadata rather than rewriting the alignment itself. The current Epithema implementation summarizes one aligned FASTA or Stockholm input as a governed table containing the overall row and column counts plus per-row ungapped-length and gap-count statistics.

## Inputs

The current v1 interface accepts exactly one local aligned FASTA or Stockholm alignment path and preserves the original row ordering in the reported table.

## Outputs

CLI and service output is a deterministic table report. The current row schema includes the alignment identifier when available, the 1-based row ordinal, the row identifier, the pairwise-versus-multiple classification, the ungapped residue count, and the gap count for each alignment row.

## Current Status

This method is implemented and exposed through `epithema infoalign`. Current Rust service coverage exercises the committed three-row Stockholm fixture and locks down the expected row count, column count, and row-level gap statistics.

## Caveats

The first release reports a conservative metadata subset only. It does not attempt broader historical EMBOSS summary fields, scoring annotations, or sequence-derived biological interpretation beyond the typed alignment statistics.

## Declared Artifacts

### Multiple-alignment Stockholm fixture

- Artifact ID: `multiple_alignment_stockholm`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/multiple_alignment.sto`
- Notes: Repository-managed Stockholm alignment fixture used to validate stable row-order metadata reporting.

## Declared Examples

### Summarize a three-row Stockholm alignment

- Example ID: `summarize_multiple_alignment_statistics`
- Description: Reports stable per-row gap and ungapped-length statistics for the committed multiple-alignment fixture.
- Referenced artifacts: `multiple_alignment_stockholm`
- Expected outputs:
  - `alignment_summary_table`: Alignment summary table (A deterministic per-row table reporting ordinals, identifiers, ungapped lengths, and gap counts.)

## Provenance

- Curated by: epithema maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `summarize_multiple_alignment_statistics`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
