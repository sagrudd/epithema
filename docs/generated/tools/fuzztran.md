# fuzztran

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Search translated nucleotide frames for deterministic protein motifs

## Document Metadata

- Document ID: `fuzztran-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `pattern_tools`
- Legacy names: `fuzztran`

## Overview

`fuzztran` translates nucleotide sequence records in the three forward reading frames and searches those translated amino-acid strings for one deterministic protein-like motif. The EMBOSS-RS v1 surface keeps the translation model intentionally narrow: standard genetic code only, forward frames 1-3 only, and no reverse-complement frame search.

## Inputs

The current v1 interface accepts one local nucleotide sequence input path and one protein-pattern string. Inputs are loaded through the shared EMBOSS-RS readers for FASTA, FASTQ, EMBL, and GenBank. Records classified as protein are rejected.

## Translation Model

Translation uses the shared EMBOSS-RS standard-code DNA frame helper. Forward frames 1, 2, and 3 are scanned. Trailing partial codons are ignored. Stop codons are translated as `*` and remain searchable as ordinary translated symbols. Ambiguous or otherwise unsupported codons currently fail translation rather than being expanded conservatively.

## Pattern Model

The protein motif syntax is shared with `fuzzpro`: exact amino-acid symbols plus `X` as a single-residue wildcard. Matching is case-insensitive. Overlapping translated matches are reported.

## Outputs

The tool emits a stable table report with one row per translated hit. Columns are `record`, `pattern`, `frame`, `aa_start`, `aa_end`, `nt_start`, `nt_end`, and `matched`. Both amino-acid and nucleotide coordinates are user-facing 1-based inclusive.

## Current Status

This method is implemented and exposed through `emboss-rs fuzztran`. Validation currently covers a forward-frame match, overlapping translated hits, no-hit behavior, invalid-pattern failure, protein-input rejection, and translation failure on ambiguous codons.

## Caveats

The first release does not implement reverse-strand scanning, alternative genetic codes, ambiguity-aware codon expansion, gaps, or the broader historical EMBOSS fuzzy-expression language.

## Declared Artifacts

### Coding nucleotide fixture

- Artifact ID: `coding_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/checktrans_nucleotide.fasta`
- Notes: Repository-managed nucleotide fixture used for deterministic translated-pattern validation.

## Declared Examples

### Search forward translated frames with a protein motif

- Example ID: `forward_frame_search`
- Description: Demonstrates deterministic forward-frame translation and motif search with amino-acid and nucleotide coordinate reporting.
- Referenced artifacts: `coding_fixture`
- Parameters:
  - `pattern` = `MA`
- Expected outputs:
  - `translated_pattern_hits`: Translated motif hits (A stable table report containing frame-aware translated hits with 1-based inclusive amino-acid and nucleotide coordinates.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Validation Intent

- Required examples: `forward_frame_search`
- Compare against legacy: no
- Require provenance capture: yes

