# maskambignuc

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Mask nucleotide ambiguity residues with N while preserving record order

## Document Metadata

- Document ID: `maskambignuc-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `feature_tools`
- Legacy names: `maskambignuc`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/maskambignuc.validation.json`](../validation/maskambignuc.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`maskambignuc` rewrites conservative nucleotide ambiguity symbols to `N` through the shared sequence-record model. The Epithema v1 implementation is deliberately narrow: it accepts nucleotide records only, preserves record order and annotations, and leaves canonical `A/C/G/T/U` residues unchanged.

## Inputs

The current tool accepts one local sequence input through the governed IO layer. FASTA is the primary exercised format in validation. Protein inputs are rejected, and ambiguity detection is limited to the common nucleotide ambiguity symbols already accepted by the shared alphabet model.

## Outputs

The tool emits a FASTA sequence collection through the shared result path. Each output record preserves the input identifier and attached metadata, and annotations remain attached in the payload because masking is done in place.

## Current Status

This method is implemented and exposed through `epithema maskambignuc`. Validation currently covers deterministic masking against a committed nucleotide FASTA fixture and explicit rejection of non-nucleotide inputs in the Rust tool layer.

## Caveats

The first release masks only the conservative ambiguity set `N/R/Y/S/W/K/M/B/D/H/V`. Gap markers and stop symbols are left unchanged, and the method does not attempt ambiguity-aware feature-coordinate reporting beyond preserving existing annotations.

## Declared Artifacts

### Ambiguous nucleotide FASTA fixture

- Artifact ID: `ambiguous_nucleotide_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/ambiguous_nucleotide_records.fasta`
- Notes: Repository-managed nucleotide FASTA fixture containing conservative ambiguity symbols for deterministic masking validation.

## Declared Examples

### Mask conservative nucleotide ambiguity symbols with N

- Example ID: `mask_nucleotide_ambiguities`
- Description: Runs `maskambignuc` against a committed FASTA fixture and rewrites ambiguity residues while leaving canonical nucleotides unchanged.
- Referenced artifacts: `ambiguous_nucleotide_fasta`
- Expected outputs:
  - `masked_nucleotide_sequences`: Ambiguity-masked nucleotide FASTA output (Stable FASTA output where the fixture residue string `ACGTRYN` becomes `ACGTNNN`.)
- Legacy reference: EMBOSS maskambignuc application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/maskambignuc.acd`
  - Invocation: `maskambignuc -sequence ambiguous_nucleotide_records.fasta -outseq stdout`

## Provenance

- Curated by: epithema maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `mask_nucleotide_ambiguities`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
