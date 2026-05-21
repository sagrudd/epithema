# makenucseq

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Create deterministic nucleotide sequence records from a bounded random generator

## Document Metadata

- Document ID: `makenucseq-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_stream`
- Legacy names: `makenucseq`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/makenucseq.validation.json`](../validation/makenucseq.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`makenucseq` creates one or more deterministic nucleotide sequence records from a documented seed and a bounded canonical alphabet. The first EMBOSS-RS release intentionally treats this as governed synthetic sequence generation rather than a broad stochastic simulation surface.

## Inputs

The current interface requires an identifier prefix and an exact residue length. Optional flags control `--count`, `--seed`, `--molecule` (`dna` or `rna`), and a shared description string attached to generated records.

## Outputs

The tool emits one or more FASTA sequence records through the shared output path. CLI summaries report the identifier prefix, length, count, seed, chosen molecule kind, and FASTA output format.

## Current Status

This method is implemented and exposed through `emboss-rs makenucseq`. Validation currently covers deterministic DNA and RNA generation, identifier-prefix behavior, and explicit molecule handling in the Rust tool and service layers.

## Caveats

The v1 surface supports only canonical DNA or RNA alphabets, exact-length generation, and deterministic seed-driven output. It does not model base frequencies, ambiguity symbols, motifs, or quality-score simulation.

## Declared Artifacts

### makenucseq governed case note

- Artifact ID: `makenucseq_rna_case`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-testkit/tests/fixtures/autodoc/makenucseq_create_rna_records_case.md`
- Notes: Repository-managed note describing the deterministic RNA generation case used for the governed `makenucseq` surface.

## Declared Examples

### Generate two deterministic RNA records

- Example ID: `generate_two_rna_records`
- Description: Builds two six-residue RNA records from a documented seed and a shared identifier prefix.
- Referenced artifacts: `makenucseq_rna_case`
- Parameters:
  - `identifier-prefix` = `made_nuc`
  - `length` = `6`
  - `count` = `2`
  - `seed` = `7`
  - `molecule` = `rna`
- Expected outputs:
  - `generated_rna_records`: Generated RNA records (Two RNA FASTA records named `made_nuc_1` and `made_nuc_2` are emitted in stable order, and the result summary reports the deterministic seed and count.)
- Legacy reference: EMBOSS makenucseq application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/makenucseq.acd`
  - Invocation: `makenucseq -name made_nuc -length 6 -number 2 -seed 7 -type rna -outseq stdout`

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `generate_two_rna_records`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
