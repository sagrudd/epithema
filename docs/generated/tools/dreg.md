# dreg

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

scan nucleotide sequences with deterministic bounded regular expressions

## Document Metadata

- Document ID: `dreg-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `pattern_tools`
- Legacy names: `dreg`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/dreg.validation.json`](../validation/dreg.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`dreg` searches nucleotide sequence records with a bounded regular expression. EMBOSS-RS v1 applies a Rust regular expression case-insensitively to normalized nucleotide residues and reports overlapping forward-strand matches only.

## Inputs

The current interface accepts one local nucleotide input and one regular-expression pattern. Protein-classified inputs are rejected. Unknown-classified residues are treated literally in the subject sequence; there is no ambiguity-expansion layer.

## Outputs

The output is a deterministic table with record identifier, pattern text, 1-based inclusive start and end coordinates, and the matched normalized residue slice.

## Current Status

This method is implemented and exposed through `emboss-rs dreg`. Validation currently covers overlapping regex hits against a committed nucleotide FASTA fixture through the Rust tool and service layers.

## Caveats

The first release does not search reverse complements, does not expose EMBOSS-era fuzzy mismatch semantics, and rejects empty or zero-width regular expressions.

## Declared Artifacts

### Dreg nucleotide FASTA fixture

- Artifact ID: `dreg_records_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/dreg_records.fasta`
- Notes: Repository-managed nucleotide FASTA fixture containing one record with overlapping `ATA` matches.

## Declared Examples

### Report overlapping forward-strand regex hits

- Example ID: `report_overlapping_regex_hits`
- Description: Runs `dreg` against a committed nucleotide FASTA fixture and reports overlapping `ATA` matches from the forward sequence.
- Referenced artifacts: `dreg_records_fasta`
- Parameters:
  - `pattern` = `ATA`
- Expected outputs:
  - `dreg_hit_table`: Overlapping nucleotide regex hit table (Two overlapping hits are reported from record `dregA` with 1-based inclusive coordinates.)
- Legacy reference: EMBOSS dreg application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/dreg.acd`
  - Invocation: `dreg -sequence dreg_records.fasta -pattern ATA -outfile stdout`

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS dreg application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/dreg.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `report_overlapping_regex_hits`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
