# einverted

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

report exact inverted-repeat arms with bounded spacer length in nucleotide sequences

## Document Metadata

- Document ID: `einverted-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `pattern_tools`
- Legacy names: `einverted`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/einverted.validation.json`](../validation/einverted.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`einverted` reports exact inverted-repeat arms in nucleotide sequence records. EMBOSS-RS v1 searches for exact reverse-complement arm pairs with a bounded spacer and reports overlapping hits deterministically.

## Inputs

The current interface accepts one local nucleotide input plus optional minimum arm length and maximum spacer length. Protein and unknown-classified inputs are rejected because the search depends on reverse-complement semantics.

## Outputs

The result is a deterministic table with record identifier, left and right arm coordinates, spacer length, arm length, and the two exact arm sequences.

## Current Status

This method is implemented and exposed through `emboss-rs einverted`. Validation currently covers exact 4-residue inverted-repeat arm detection with bounded spacer length against a committed nucleotide FASTA fixture.

## Caveats

The first release does not score mismatches, bulges, or thermodynamic stability. It reports exact arm pairs only and leaves higher-order ranking for future work.

## Declared Artifacts

### Einverted nucleotide FASTA fixture

- Artifact ID: `einverted_records_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/einverted_records.fasta`
- Notes: Repository-managed nucleotide FASTA fixture containing exact inverted-repeat arms with bounded spacer length.

## Declared Examples

### Report exact inverted-repeat arms with bounded spacer length

- Example ID: `report_exact_inverted_repeat_arms`
- Description: Runs `einverted` against a committed nucleotide FASTA fixture and reports deterministic exact reverse-complement arm pairs with a minimum arm length of 4 and a maximum spacer length of 2.
- Referenced artifacts: `einverted_records_fasta`
- Parameters:
  - `min_arm_length` = `4`
  - `max_gap_length` = `2`
- Expected outputs:
  - `einverted_hit_table`: Exact inverted-repeat hit table (The committed fixture yields deterministic overlapping exact arm-pair rows, including one 4-residue arm pair with a 2-residue spacer.)
- Legacy reference: EMBOSS einverted application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/einverted.acd`
  - Invocation: `einverted -sequence einverted_records.fasta -threshold 0 -gap 2 -match 1 -mismatch -1 -outfile stdout`

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS einverted application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/einverted.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `report_exact_inverted_repeat_arms`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
