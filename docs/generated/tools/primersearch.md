# primersearch

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report deterministic complete primer-pair hits against local nucleotide sequence inputs

## Document Metadata

- Document ID: `primersearch-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `primer_tools`
- Legacy names: `primersearch`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/primersearch.validation.json`](../validation/primersearch.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`primersearch` is the bounded primer-pair search member of the active primer and assay-oriented rework program. The EMBOSS-RS v1 surface searches one local nucleotide sequence input against one local tab-delimited primer-pair file and returns deterministic complete-pair hit rows as a stable table-first report.

## Inputs

The current interface accepts exactly two local inputs: one nucleotide sequence file and one primer-pair TSV. Each non-empty primer row must contain exactly three tab-delimited fields: pair name, forward primer, and reverse primer. Provider-backed sequence acquisition, inline literals, and non-nucleotide targets remain outside the bounded v1 seam.

## Outputs

The result is a stable normalized table with record identifier, primer-pair name, strand/orientation, one-based inclusive left and right primer coordinates, one-based inclusive amplicon coordinates and length, and the matched left and right primer slices. The bounded seam reports complete pair hits only and keeps all coordinates tied to the same matching path.

## Legacy Context

This bounded release keeps one historical `primersearch` user need in scope while modernizing around deterministic primer-hit table reporting rather than primer-design optimization or broader assay-ranking behavior. The governed validation seam now includes a canonical compared fixture for the bounded local target-plus-primer-file path.

## Current Status

This method is implemented and exposed through `emboss-rs primersearch`. The governed surface ships with a checked-in validation stub, curated legacy provenance, Rust coverage for the bounded local target-plus-primer-file path, and a canonical compared analytical fixture covering stable normalized primer-hit rows.

## Caveats

The v1 `primersearch` seam is intentionally narrow. It does not perform primer-design optimization, thermodynamic scoring, assay ranking, or generalized primer-analysis orchestration, and it does not widen into `eprimer3` or `sirna` merely because this bounded search slice ships.

## Declared Artifacts

### Primersearch nucleotide target fixture

- Artifact ID: `primersearch_targets_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/primersearch_targets.fasta`
- Notes: Repository-managed nucleotide targets used for deterministic bounded primersearch validation.

### Primersearch primer-pair fixture

- Artifact ID: `primersearch_pairs_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/primersearch_pairs.tsv`
- Notes: Repository-managed primer-pair rows used for deterministic bounded primersearch validation.

## Declared Examples

### Search one local nucleotide input for deterministic complete primer-pair hits

- Example ID: `primersearch_hits_example`
- Description: Executes the bounded primersearch seam against the committed target FASTA and primer-pair TSV fixtures to emit stable normalized hit rows.
- Referenced artifacts: `primersearch_targets_fixture`, `primersearch_pairs_fixture`
- Expected outputs:
  - `primersearch_table`: Bounded primersearch analytical table (Stable normalized complete-pair hit rows with strand, primer coordinates, amplicon coordinates, and matched primer slices.)
- Legacy reference: EMBOSS primersearch application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/primersearch.acd`
  - Invocation: `primersearch -seqall primersearch_targets.fasta -infile primersearch_pairs.tsv -outfile stdout`

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS primersearch application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/primersearch.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `primersearch_hits_example`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes
