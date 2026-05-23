# tranalign

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Project aligned protein rows onto matching codon alignments

## Document Metadata

- Document ID: `tranalign-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `translation_tools`
- Legacy names: `tranalign`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/tranalign.validation.json`](../validation/tranalign.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`tranalign` projects an aligned protein set onto matching nucleotide coding sequences to produce a codon alignment. EMBOSS-RS v1 keeps the compatibility rules strict: rows are paired to coding sequences by exact identifier, translation is checked in frame 1 under the standard genetic code, and protein gaps expand to triple-nucleotide gaps.

## Inputs

The interface accepts one protein alignment input in aligned FASTA or Stockholm plus one nucleotide coding-sequence input. Each protein alignment row must have an exact identifier match in the nucleotide input, and the ungapped protein row must match the strict frame-1 translation of the corresponding coding sequence after terminal-stop normalization.

## Outputs

The result payload is a Stockholm codon alignment that preserves the input protein-row order. Protein gaps become `---`, and each non-gap protein symbol consumes exactly one source codon from the matching nucleotide record.

## Legacy Context

This acceptance anchor keeps one historical-style `tranalign` invocation in view and compares the EMBOSS-RS Stockholm payload against a committed expected output. The governed comparison validates exact identifier pairing and codon-projection stability for the committed alignment fixture.

## Current Status

This method is implemented and exposed through `emboss-rs tranalign`. Current validation covers codon projection from the committed protein alignment and coding-sequence fixtures, exact identifier pairing, and strict translation compatibility.

## Caveats

The first release requires exact identifier matches, strict frame-1 compatibility, and a complete codon for every aligned non-gap residue. It does not infer alternative pairings, reverse-complement frames, or broader codon-projection heuristics.

## Declared Artifacts

### Protein alignment fixture

- Artifact ID: `tranalign_protein_alignment_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/tranalign_protein_alignment.sto`
- Notes: Repository-managed protein Stockholm alignment used for deterministic codon projection validation.

### Matching coding-sequence fixture

- Artifact ID: `checktrans_nucleotide_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/checktrans_nucleotide.fasta`
- Notes: Repository-managed coding-sequence fixture paired by exact identifier with the committed protein alignment rows.

## Declared Examples

### Project a protein alignment onto codon rows

- Example ID: `project_protein_alignment_to_codons`
- Description: Projects the committed protein Stockholm alignment onto matching coding-sequence records and emits a deterministic codon alignment as Stockholm.
- Referenced artifacts: `tranalign_protein_alignment_fixture`, `checktrans_nucleotide_fixture`
- Expected outputs:
  - `codon_alignment_stockholm`: Projected codon Stockholm alignment (A stable Stockholm rendering of the projected codon alignment.)
- Legacy reference: EMBOSS tranalign application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/tranalign.acd`
  - Invocation: `tranalign -asequence tranalign_protein_alignment.sto -bsequence checktrans_nucleotide.fasta -outseq stdout`

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS tranalign application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/tranalign.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `project_protein_alignment_to_codons`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes
