# recoder

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report synonymous single-codon edits that remove exact forward-strand restriction sites

## Document Metadata

- Document ID: `recoder-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `restriction_tools`
- Legacy names: `recoder`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/recoder.validation.json`](../validation/recoder.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`recoder` reports conservative synonymous single-codon edits that remove an exact canonical DNA restriction site from a coding sequence while preserving translation. Epithema v1 treats this as a bounded analytical design aid rather than an enzyme-database or sequence-optimization system.

## Inputs

The current interface accepts one local coding-DNA input path plus one canonical DNA site string. Inputs must validate as strict coding sequences under the standard genetic code, with canonical DNA codons and no internal stops. RNA residues, ambiguous nucleotides, and non-coding inputs are rejected.

## Outputs

The tool emits a stable table report with columns `record`, `site`, `occurrence`, `site_start`, `site_end`, `codon_index`, `codon_start`, `codon_end`, `amino_acid`, `original_codon`, `replacement_codon`, and `mutated_sequence`.

## Model

This first release considers only exact forward-strand site matches and only single-codon synonymous substitutions. A candidate is reported only when the target site occurrence is removed and total exact site count decreases after the edit.

## Current Status

This method is implemented and exposed through `epithema recoder`. Validation currently covers deterministic synonymous removal candidates against a committed coding-DNA fixture and non-coding-input rejection.

## Caveats

The first release does not model enzyme databases, reverse-complement matching, multi-edit optimization, codon-usage optimization, or feature-coordinate remapping. It is intentionally limited to canonical DNA site strings of length at least four.

## Declared Artifacts

### Restriction-site removal coding fixture

- Artifact ID: `recoder_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/recoder_records.fasta`
- Notes: Repository-managed coding-DNA fixture containing one exact EcoRI site in-frame.

## Declared Examples

### Report synonymous edits that remove an EcoRI site

- Example ID: `remove_exact_ecori_site_with_synonymous_edits`
- Description: Loads a committed coding-DNA fixture and reports deterministic single-codon synonymous edits that remove the exact site `GAATTC`.
- Referenced artifacts: `recoder_fixture`
- Parameters:
  - `site` = `GAATTC`
- Expected outputs:
  - `recoder_candidates`: Restriction-site removal candidate table (A stable tabular report containing one row per synonymous single-codon edit that removes the exact target site.)
- Legacy reference: EMBOSS recoder application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/recoder.acd`
  - Invocation: `recoder -sequence recoder_records.fasta -site GAATTC -outfile stdout`

## Provenance

- Curated by: OpenAI Codex
- Source references:
  - EMBOSS recoder application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/recoder.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `remove_exact_ecori_site_with_synonymous_edits`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
