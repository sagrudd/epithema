# fuzznuc

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Search nucleotide sequences for deterministic exact or IUPAC-ambiguous motifs

## Document Metadata

- Document ID: `fuzznuc-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `pattern_tools`
- Legacy names: `fuzznuc`

## Overview

`fuzznuc` searches nucleotide sequence records for one forward-strand motif and reports all matches in stable input order. The EMBOSS-RS v1 surface keeps the pattern model intentionally narrow: exact nucleotide symbols plus IUPAC ambiguity codes, no gaps, no indels, and no reverse-strand search.

## Inputs

The current v1 interface accepts one local nucleotide sequence input path and one motif string. Inputs are loaded through the shared EMBOSS-RS readers for FASTA, FASTQ, EMBL, and GenBank. Records classified as protein are rejected.

## Pattern Model

Supported motif syntax is exact nucleotide text plus standard IUPAC ambiguity symbols such as `N`, `R`, `Y`, `W`, and `S`. Pattern and subject sequences are normalized case-insensitively. Overlapping matches are reported. Subject ambiguity symbols also participate conservatively, so a subject symbol only matches when its possible residues are a subset of the pattern symbol set.

## Outputs

The tool emits a stable table report with one row per hit. Columns are `record`, `pattern`, `strand`, `start`, `end`, and `matched`. Coordinates are user-facing 1-based inclusive. The current strand policy is always `forward`.

## Current Status

This method is implemented and exposed through `emboss-rs fuzznuc`. Validation currently covers exact and ambiguity-aware matching, overlapping matches, no-hit behavior, invalid-pattern failure, and protein-input rejection.

## Caveats

The first release does not implement the historical EMBOSS fuzzy-expression language, mismatches, gaps, or reverse-complement scanning. Empty patterns are rejected during parsing.

## Declared Artifacts

### Nucleotide pattern fixture

- Artifact ID: `nucleotide_pattern_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/nucleotide_pattern_records.fasta`
- Notes: Repository-managed FASTA fixture used for deterministic nucleotide motif validation.

## Declared Examples

### Search a nucleotide fixture with an IUPAC motif

- Example ID: `iupac_forward_search`
- Description: Demonstrates deterministic forward-only searching with an ambiguity-aware nucleotide pattern and stable hit ordering.
- Referenced artifacts: `nucleotide_pattern_fixture`
- Parameters:
  - `pattern` = `ACGN`
- Expected outputs:
  - `nucleotide_pattern_hits`: Forward-strand nucleotide motif hits (A stable table report containing one row per ambiguity-aware match with 1-based inclusive coordinates.)

## Provenance

- Curated by: Codex
- Source references: none declared

## Validation Intent

- Required examples: `iupac_forward_search`
- Compare against legacy: no
- Require provenance capture: yes

