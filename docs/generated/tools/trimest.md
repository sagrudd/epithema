# trimest

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Remove terminal poly-A tails from nucleotide sequence records

## Document Metadata

- Document ID: `trimest-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_edit`
- Legacy names: `trimest`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/trimest.validation.json`](../validation/trimest.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`trimest` removes bounded terminal poly-A tails from nucleotide sequence records. The EMBOSS-RS v1 surface trims only trailing 3' runs of `A` when they meet a configured minimum length, keeping the behavior explicit and reproducible.

## Inputs

The current interface accepts one local nucleotide sequence input and an optional `--min-tail` threshold. Input records must be nucleotide-like; protein records are rejected.

## Outputs

The tool emits trimmed FASTA records in input order. CLI summaries report the source input, minimum tail threshold, exact trim rule, and FASTA output format.

## Current Status

This method is implemented and exposed through `emboss-rs trimest`. Validation currently covers deterministic terminal poly-A trimming against committed FASTA fixtures in the Rust tool and service layers.

## Caveats

The v1 implementation trims trailing `A` runs only. It does not attempt bidirectional EST cleanup, adapter detection, internal homopolymer trimming, or annotation-coordinate remapping, and it drops feature annotations after editing.

## Declared Artifacts

### trimest FASTA fixture

- Artifact ID: `trimest_records_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/trimest_records.fasta`
- Notes: Repository-managed FASTA input fixture used for deterministic terminal poly-A trimming validation.

## Declared Examples

### Trim terminal poly-A tails when they meet the minimum length

- Example ID: `trim_terminal_poly_a`
- Description: Runs `trimest` against the committed FASTA fixture with `--min-tail 4`.
- Referenced artifacts: `trimest_records_fasta`
- Expected outputs:
  - `trimest_output_records`: Trimmed nucleotide records (The output FASTA records contain residues `ACGT` and `TTGCAAA`, showing that only the first record met the four-residue trailing poly-A threshold.)
- Legacy reference: EMBOSS trimest application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/trimest.acd`
  - Invocation: `trimest -sequence trimest_records.fasta -minpoly 4 -outseq stdout`

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `trim_terminal_poly_a`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
