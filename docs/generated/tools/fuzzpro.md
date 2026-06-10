# fuzzpro

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Search protein sequences for deterministic exact or wildcard motifs

## Document Metadata

- Document ID: `fuzzpro-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `pattern_tools`
- Legacy names: `fuzzpro`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/fuzzpro.validation.json`](../validation/fuzzpro.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`fuzzpro` searches protein sequence records for one linear motif and reports all matches in stable input order. The Epithema v1 surface keeps the pattern model intentionally narrow: exact amino-acid symbols plus `X` as a one-residue wildcard, no gaps, no indels, and no full historical EMBOSS fuzzy-expression language.

## Inputs

The current v1 interface accepts one local protein sequence input path and one motif string. Inputs are loaded through the shared Epithema readers for FASTA, FASTQ, EMBL, and GenBank. Records classified as nucleotide are rejected.

## Pattern Model

Supported motif syntax is exact amino-acid text plus `X` as a single-residue wildcard. Pattern and subject sequences are normalized case-insensitively. Overlapping matches are reported. Subject residues are interpreted literally in v1; there is no protein ambiguity-class expansion beyond the wildcard.

## Outputs

The tool emits a stable table report with one row per hit. Columns are `record`, `pattern`, `start`, `end`, and `matched`. Coordinates are user-facing 1-based inclusive.

## Current Status

This method is implemented and exposed through `epithema fuzzpro`. Validation currently covers exact and wildcard matching, overlapping matches, no-hit behavior, invalid-pattern failure, and nucleotide-input rejection.

## Caveats

The first release does not implement character classes, mismatches, gaps, or the broader historical EMBOSS fuzzy-pattern language. Empty patterns are rejected during parsing.

## Declared Artifacts

### Protein pattern fixture

- Artifact ID: `protein_pattern_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/protein_records.fasta`
- Notes: Repository-managed FASTA fixture used for deterministic protein motif validation.

## Declared Examples

### Search a protein fixture with a wildcard motif

- Example ID: `wildcard_forward_search`
- Description: Demonstrates deterministic protein motif searching with the `X` one-residue wildcard and stable hit ordering.
- Referenced artifacts: `protein_pattern_fixture`
- Parameters:
  - `pattern` = `MX`
- Expected outputs:
  - `protein_pattern_hits`: Protein motif hits (A stable table report containing one row per protein motif hit with 1-based inclusive coordinates.)

## Provenance

- Curated by: epithema maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `wildcard_forward_search`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
