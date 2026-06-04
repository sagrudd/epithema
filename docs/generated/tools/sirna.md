# sirna

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report deterministic bounded siRNA-candidate rows against one local nucleotide input

## Document Metadata

- Document ID: `sirna-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `primer_tools`
- Legacy names: `sirna`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/sirna.validation.json`](../validation/sirna.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`sirna` is the bounded siRNA-candidate discovery member of the active primer and assay-oriented rework program. The EMBOSS-RS v1 surface analyzes one local nucleotide sequence input and returns deterministic candidate rows as a stable table-first report rather than a generalized RNAi-efficacy or off-target ranking workflow.

## Inputs

The current interface accepts exactly one local nucleotide sequence file. Provider-backed acquisition, inline literals, generalized parameter tuning, and non-nucleotide inputs remain outside the bounded v1 seam. The shipped surface uses one governed default candidate-selection parameter set carried through the same computation path.

## Outputs

The result is a stable normalized table with record identifier, candidate identifier, strand/orientation, one-based inclusive target interval, duplex length, sense and guide sequences, canonical and ambiguous symbol counts, GC fraction, guide 5' base, guide-seed A/U count, and maximum homopolymer run. All fields come from the same deterministic local candidate-selection path.

## Legacy Context

This bounded release keeps one historical `sirna` user need in scope while modernizing around deterministic local candidate reporting instead of generalized RNAi-efficacy prediction, transcriptome-wide off-target search, or broad small-RNA ranking behavior. The governed validation seam now includes a canonical compared fixture covering stable normalized siRNA-candidate rows for the bounded local nucleotide-input path.

## Current Status

This method is implemented and exposed through `emboss-rs sirna`. The governed surface ships with a checked-in validation stub, curated legacy provenance, Rust coverage for the bounded local nucleotide-input path, and a canonical compared analytical fixture covering stable normalized siRNA-candidate rows.

## Caveats

The v1 `sirna` seam is intentionally narrow. It does not perform generalized RNAi-efficacy prediction, transcriptome-wide off-target search, or small-RNA ranking orchestration, and it does not widen into command discovery and help-navigation merely because this bounded primer-family slice ships.

## Declared Artifacts

### Sirna nucleotide target fixture

- Artifact ID: `sirna_targets_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/sirna_targets.fasta`
- Notes: Repository-managed nucleotide targets used for deterministic bounded sirna validation.

## Declared Examples

### Generate deterministic bounded siRNA candidates from one local nucleotide input

- Example ID: `sirna_candidates_example`
- Description: Executes the bounded sirna seam against the committed nucleotide fixture to emit stable normalized candidate rows.
- Referenced artifacts: `sirna_targets_fixture`
- Expected outputs:
  - `sirna_table`: Bounded sirna analytical table (Stable normalized siRNA candidate rows with target interval, duplex sequences, GC fraction, guide 5' base, guide-seed A/U count, and maximum homopolymer run.)
- Legacy reference: EMBOSS sirna application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/sirna.acd`
  - Invocation: `sirna -sequence sirna_targets.fasta -outfile stdout`

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS sirna application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/sirna.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `sirna_candidates_example`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes
