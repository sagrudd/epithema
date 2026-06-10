# eprimer3

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report deterministic bounded primer-and-oligo design candidates against one local nucleotide input

## Document Metadata

- Document ID: `eprimer3-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `primer_tools`
- Legacy names: `eprimer3`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/eprimer3.validation.json`](../validation/eprimer3.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`eprimer3` is the bounded primer-and-oligo design member of the active primer and assay-oriented rework program. The Epithema v1 surface analyzes one local nucleotide sequence input and returns deterministic candidate rows as a stable table-first report rather than a generalized assay-ranking workflow.

## Inputs

The current interface accepts exactly one local nucleotide sequence file. Provider-backed acquisition, inline literals, generalized parameter tuning, and non-nucleotide inputs remain outside the bounded v1 seam. The shipped surface uses one governed default design-parameter set carried through the same computation path.

## Outputs

The result is a stable normalized table with record identifier, candidate identifier, strand/orientation, one-based inclusive oligo coordinates, oligo length and sequence, canonical and ambiguous symbol counts, GC fraction, conservative melting estimate, and 3'-terminal GC count. All fields come from the same deterministic local candidate-generation path.

## Legacy Context

This bounded release keeps one historical `eprimer3` user need in scope while modernizing around deterministic local candidate reporting instead of broad thermodynamic optimization or generalized assay-ranking behavior. The governed validation seam now includes a canonical compared fixture covering stable normalized candidate rows for the bounded local nucleotide-input path.

## Current Status

This method is implemented and exposed through `epithema eprimer3`. The governed surface ships with a checked-in validation stub, curated legacy provenance, Rust coverage for the bounded local nucleotide-input path, and a canonical compared analytical fixture covering stable normalized candidate rows.

## Caveats

The v1 `eprimer3` seam is intentionally narrow. It does not perform generalized assay ranking, broad thermodynamic optimization, or family-wide primer-analysis orchestration, and it does not widen into `sirna` merely because this bounded primer-and-oligo design slice ships.

## Declared Artifacts

### Eprimer3 nucleotide target fixture

- Artifact ID: `eprimer3_targets_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/eprimer3_targets.fasta`
- Notes: Repository-managed nucleotide targets used for deterministic bounded eprimer3 validation.

## Declared Examples

### Generate deterministic bounded primer candidates from one local nucleotide input

- Example ID: `eprimer3_candidates_example`
- Description: Executes the bounded eprimer3 seam against the committed nucleotide fixture to emit stable normalized candidate rows.
- Referenced artifacts: `eprimer3_targets_fixture`
- Expected outputs:
  - `eprimer3_table`: Bounded eprimer3 analytical table (Stable normalized primer-and-oligo candidate rows with strand, interval, sequence, GC fraction, conservative Tm, and 3'-terminal GC count.)
- Legacy reference: EMBOSS eprimer3 application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/eprimer3.acd`
  - Invocation: `eprimer3 -sequence eprimer3_targets.fasta -outfile stdout`

## Provenance

- Curated by: epithema maintainers
- Source references:
  - EMBOSS eprimer3 application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/eprimer3.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `eprimer3_candidates_example`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes
