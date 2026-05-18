# preg

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

scan protein sequences with deterministic bounded regular expressions

## Document Metadata

- Document ID: `preg-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `pattern_tools`
- Legacy names: `preg`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/preg.validation.json`](../validation/preg.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`preg` searches protein sequence records with a bounded regular-expression model and reports overlapping forward hits in a stable table. EMBOSS-RS v1 uses Rust-regex-compatible ASCII expressions rather than the full historical EMBOSS search language.

## Inputs

The current interface accepts one local protein input and one regular-expression pattern. The subject may contain one or more records. The pattern must compile successfully, must not be empty, and must consume at least one residue.

## Outputs

The output is a stable table with one row per hit. Each row reports the source record, the searched pattern, 1-based inclusive coordinates, and the matched normalized sequence text.

## Current Status

This method is implemented and exposed through `emboss-rs preg`. Validation currently covers overlapping regex hits through the Rust tool and service layers.

## Caveats

The first release is protein-only, forward-only, and bounded to Rust-regex-compatible ASCII expressions. It does not implement reverse scanning, database retrieval, or EMBOSS-era expression aliases beyond what the regex engine already supports.

## Declared Artifacts

### Preg FASTA fixture

- Artifact ID: `preg_records_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/preg_records.fasta`
- Notes: Repository-managed protein FASTA fixture used for bounded regex searching.

## Declared Examples

### Report overlapping protein regex hits

- Example ID: `report_overlapping_protein_regex_hits`
- Description: Runs `preg` against a committed protein FASTA fixture and reports overlapping hits for the expression `MAM`.
- Referenced artifacts: `preg_records_fasta`
- Parameters:
  - `pattern` = `MAM`
- Expected outputs:
  - `preg_hit_table`: Protein regex hit table (Two overlapping hits are reported in the first fixture record.)
- Legacy reference: EMBOSS preg application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/preg.acd`
  - Invocation: `preg -sequence preg_records.fasta -pattern MAM -outfile stdout`

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS preg application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/preg.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `report_overlapping_protein_regex_hits`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes

