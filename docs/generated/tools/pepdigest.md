# pepdigest

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report deterministic full-digest peptide fragments for a small typed protease set

## Document Metadata

- Document ID: `pepdigest-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_stats`
- Legacy names: `pepdigest`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/pepdigest.validation.json`](../validation/pepdigest.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`pepdigest` reports full-digest peptide fragments for each protein record. The Epithema v1 implementation is intentionally narrow: it supports a small typed protease set, uses exact deterministic cleavage rules, and emits one stable row per resulting peptide.

## Inputs

The current interface accepts one local protein input path plus an optional protease selector. Inputs are loaded through the shared Epithema readers for FASTA, FASTQ, EMBL, and GenBank. Explicit nucleotide inputs are rejected. Unsupported ambiguous protein residues are rejected.

## Outputs

The tool emits a stable table report with columns `record`, `protease`, `peptide_index`, `start`, `end`, `cleavage_after`, and `sequence`. Coordinates are 1-based and inclusive.

## Digest Model

The first release supports `trypsin`, `lys-c`, `arg-c`, and `cnbr`. Trypsin cleaves after `K` or `R` unless the following residue is `P`. Lys-C cleaves after `K`. Arg-C cleaves after `R`. CNBr cleaves after `M`. Only full deterministic digestion is supported; missed cleavages, partial digest searches, and mass filters are deferred.

## Current Status

This method is implemented and exposed through `epithema pepdigest`. Validation currently covers tryptic digestion, CNBr reagent mode, stable per-peptide coordinate reporting, and nucleotide-input rejection.

## Caveats

The first release does not model missed cleavages, semi-specific digestion, post-translational modification masses, or protease mixtures. It reports only deterministic full-digest fragments.

## Declared Artifacts

### Protein digest fixture

- Artifact ID: `pepdigest_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/pepdigest_records.fasta`
- Notes: Repository-managed protein fixture used to validate deterministic full-digest peptide reporting.

## Declared Examples

### Digest protein records with trypsin

- Example ID: `digest_proteins_with_trypsin`
- Description: Loads a small protein fixture and reports deterministic tryptic peptide fragments in source order.
- Referenced artifacts: `pepdigest_fixture`
- Expected outputs:
  - `pepdigest_trypsin_table`: Tryptic peptide table (A stable peptide-fragment table with record order, peptide ordinals, and source coordinates.)
- Legacy reference: EMBOSS pepdigest application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/pepdigest.acd`
  - Invocation: `pepdigest -sequence pepdigest_records.fasta -outfile stdout`

### Digest protein records with CNBr

- Example ID: `digest_proteins_with_cnbr`
- Description: Uses the same protein fixture with the v1 `cnbr` reagent mode to validate alternate deterministic cleavage boundaries.
- Referenced artifacts: `pepdigest_fixture`
- Parameters:
  - `protease` = `cnbr`
- Expected outputs:
  - `pepdigest_cnbr_table`: CNBr peptide table (A stable peptide-fragment table reflecting cleavage after methionine residues.)
- Legacy reference: EMBOSS pepdigest application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/pepdigest.acd`
  - Invocation: `pepdigest -sequence pepdigest_records.fasta -menu cnbr -outfile stdout`

## Provenance

- Curated by: OpenAI Codex
- Source references:
  - EMBOSS pepdigest application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/pepdigest.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `digest_proteins_with_trypsin`, `digest_proteins_with_cnbr`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
