# extractalign

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Extract rows and an optional 1-based inclusive column range from an alignment

## Document Metadata

- Document ID: `extractalign-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `alignment_tools`
- Legacy names: `extractalign`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/extractalign.validation.json`](../validation/extractalign.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`extractalign` subsets an alignment by row and, optionally, by column range using explicit typed coordinates instead of ad hoc text slicing. The current implementation can retain rows selected by 1-based ordinal and/or row identifier, then optionally crop the surviving alignment to a 1-based inclusive column span.

## Inputs

The current v1 tool accepts one local aligned FASTA or Stockholm input plus optional selectors: repeated `--row <ordinal>`, repeated `--row-id <identifier>`, and a paired `--start <column>` / `--end <column>` column range. If no row selectors are supplied, all rows are retained. If no column range is supplied, all columns are retained.

## Outputs

The result is an extracted alignment payload with preserved row ordering from the source alignment after selection. Column coordinates are 1-based inclusive in the user-facing CLI and are validated strictly before extraction.

## Current Status

This method is implemented and exposed through `epithema extractalign`. Current Rust service coverage exercises mixed row-identifier and row-ordinal selection plus 1-based inclusive column slicing against the committed multiple-alignment fixture.

## Caveats

Both `--start` and `--end` must be provided together. Out-of-range ordinals, unknown identifiers, and invalid column coordinates fail clearly instead of being clipped or ignored.

## Declared Artifacts

### Multiple-alignment Stockholm fixture

- Artifact ID: `multiple_alignment_stockholm`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/multiple_alignment.sto`
- Notes: Repository-managed Stockholm alignment fixture used to validate row and column extraction behavior.

## Declared Examples

### Extract selected rows and a column slice from a Stockholm alignment

- Example ID: `extract_selected_rows_and_columns`
- Description: Retains row identifier `alpha` and row ordinal `3`, then crops the resulting alignment to columns 2 through 4 inclusive.
- Referenced artifacts: `multiple_alignment_stockholm`
- Parameters:
  - `row_id` = `alpha`
  - `row` = `3`
  - `start` = `2`
  - `end` = `4`
- Expected outputs:
  - `extracted_alignment`: Extracted alignment payload (A two-row, three-column alignment containing the selected rows and the requested inclusive column slice.)

## Provenance

- Curated by: epithema maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `extract_selected_rows_and_columns`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
