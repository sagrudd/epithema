# transeq

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Translate nucleotide sequences in forward reading frames

## Document Metadata

- Document ID: `transeq-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `translation_tools`
- Legacy names: `transeq`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/transeq.validation.json`](../validation/transeq.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`transeq` translates nucleotide sequence records in forward reading frames using the shared EMBOSS-RS standard genetic code. The v1 surface supports frame `1`, `2`, `3`, or `all`, emits protein FASTA records deterministically, and ignores trailing partial codons.

## Inputs

The current interface accepts one nucleotide input path plus optional `--frame <1|2|3|all>`. Protein-classified input is rejected clearly. Translation is strict with respect to unsupported codons and residues.

## Outputs

The tool emits one protein FASTA record per translated frame in stable source order. Output identifiers are suffixed as `<accession>.frameN`, descriptions are annotated with `translated protein frame N`, and molecule kind becomes `protein`.

## Legacy Context

This acceptance anchor keeps one historical-style `transeq` invocation in view and compares the EMBOSS-RS FASTA payload against a committed expected output. The governed comparison validates strict forward frame-1 translation for the committed coding fixture.

## Current Status

This method is implemented and exposed through `emboss-rs transeq`. Current validation covers forward frame selection, strict translation, trailing partial-codon truncation, and stable FASTA emission from the committed coding fixture.

## Caveats

The first release supports forward frames only and the standard genetic code only. It does not search reverse-complement frames, and ambiguous unsupported codons fail clearly instead of being expanded.

## Declared Artifacts

### Forward translation nucleotide fixture

- Artifact ID: `checktrans_nucleotide_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/checktrans_nucleotide.fasta`
- Notes: Repository-managed coding-sequence fixture used for deterministic forward translation validation.

## Declared Examples

### Translate a coding-sequence fixture in frame 1

- Example ID: `forward_frame_one_translation`
- Description: Translates the committed coding-sequence fixture in forward frame 1 and emits deterministic protein FASTA output.
- Referenced artifacts: `checktrans_nucleotide_fixture`
- Parameters:
  - `frame` = `1`
- Expected outputs:
  - `translated_protein_fasta`: Frame-1 translated FASTA (A stable FASTA rendering of the translated protein records for the committed coding fixture.)
- Legacy reference: EMBOSS transeq application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/transeq.acd`
  - Invocation: `transeq -sequence checktrans_nucleotide.fasta -frame 1 -outseq stdout`

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS transeq application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/transeq.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `forward_frame_one_translation`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes
