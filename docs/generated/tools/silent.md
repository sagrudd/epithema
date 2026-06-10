# silent

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report synonymous single-codon edits that create exact forward-strand restriction sites

## Document Metadata

- Document ID: `silent-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `restriction_tools`
- Legacy names: `silent`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/silent.validation.json`](../validation/silent.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`silent` reports conservative synonymous single-codon edits that create an exact canonical DNA restriction site in a coding sequence while preserving translation. Epithema v1 treats this as a bounded analytical design aid rather than an enzyme-database or sequence-optimization system.

## Inputs

The current interface accepts one local coding-DNA input path plus one canonical DNA site string. Inputs must validate as strict coding sequences under the standard genetic code, with canonical DNA codons and no internal stops. RNA residues, ambiguous nucleotides, and non-coding inputs are rejected.

## Outputs

The tool emits a stable table report with columns `record`, `site`, `site_start`, `site_end`, `codon_index`, `codon_start`, `codon_end`, `amino_acid`, `original_codon`, `replacement_codon`, and `mutated_sequence`.

## Model

This first release considers only exact forward-strand site matches and only single-codon synonymous substitutions. A candidate is reported only when the edit creates one new exact site occurrence that was absent from the original sequence.

## Current Status

This method is implemented and exposed through `epithema silent`. Validation currently covers deterministic synonymous creation candidates against a committed coding-DNA fixture and non-coding-input rejection.

## Caveats

The first release does not model enzyme databases, reverse-complement matching, multi-edit optimization, codon-usage optimization, or feature-coordinate remapping. It is intentionally limited to canonical DNA site strings of length at least four.

## Declared Artifacts

### Restriction-site creation coding fixture

- Artifact ID: `silent_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/silent_records.fasta`
- Notes: Repository-managed coding-DNA fixture that is one synonymous codon edit away from an exact EcoRI site.

## Declared Examples

### Report a synonymous edit that creates an EcoRI site

- Example ID: `create_exact_ecori_site_with_synonymous_edit`
- Description: Loads a committed coding-DNA fixture and reports deterministic single-codon synonymous edits that create the exact site `GAATTC`.
- Referenced artifacts: `silent_fixture`
- Parameters:
  - `site` = `GAATTC`
- Expected outputs:
  - `silent_candidates`: Restriction-site creation candidate table (A stable tabular report containing one row per synonymous single-codon edit that creates the exact target site.)
- Legacy reference: EMBOSS silent application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/silent.acd`
  - Invocation: `silent -sequence silent_records.fasta -site GAATTC -outfile stdout`

## Provenance

- Curated by: OpenAI Codex
- Source references:
  - EMBOSS silent application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/silent.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `create_exact_ecori_site_with_synonymous_edit`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
