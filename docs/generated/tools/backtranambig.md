# backtranambig

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Back-translate protein records into deterministic IUPAC-ambiguous DNA codons

## Document Metadata

- Document ID: `backtranambig-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `translation_tools`
- Legacy names: `backtranambig`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/backtranambig.validation.json`](../validation/backtranambig.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`backtranambig` back-translates protein sequence records into deterministic IUPAC-ambiguous DNA codons under the shared EMBOSS-RS standard genetic code. The v1 surface preserves ambiguity explicitly instead of selecting a single representative codon.

## Inputs

The current interface accepts one local protein-oriented sequence input path. Explicit nucleotide-classified input records are rejected clearly. Unknown-classified records remain acceptable when their residue content is protein-like.

## Outputs

The tool emits one DNA FASTA record per input protein record in stable order. Output identifiers are preserved, descriptions are annotated with the suffix `ambiguous back-translated DNA`, and stop symbols are rendered as `TAR` in the first release.

## Legacy Context

This acceptance anchor keeps one historical-style `backtranambig` invocation in view and compares the EMBOSS-RS FASTA payload against a committed expected output. The governed comparison focuses on deterministic IUPAC ambiguity encoding rather than broad codon-table configurability.

## Current Status

This method is implemented and exposed through `emboss-rs backtranambig`. Current validation covers the deterministic ambiguous-codon policy, stop-symbol handling with `* -> TAR`, and stable FASTA output from the committed protein fixture.

## Caveats

The first release exposes only the standard genetic code and one fixed ambiguity mapping per residue. It does not infer organism-specific codon usage or alternate code tables, and unsupported residues fail clearly.

## Declared Artifacts

### Protein ambiguous back-translation fixture

- Artifact ID: `protein_stats_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/protein_stats_records.fasta`
- Notes: Repository-managed protein fixture used for deterministic ambiguous back-translation validation.

## Declared Examples

### Back-translate protein records into ambiguous DNA codons

- Example ID: `ambiguous_backtranslation`
- Description: Back-translates the committed protein fixture into IUPAC-ambiguous DNA codons using the governed EMBOSS-RS standard-code ambiguity policy.
- Referenced artifacts: `protein_stats_fixture`
- Expected outputs:
  - `ambiguous_dna_sequences`: Ambiguous DNA FASTA output (A stable FASTA rendering of the deterministic ambiguous DNA back-translations.)
- Legacy reference: EMBOSS backtranambig application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/backtranambig.acd`
  - Invocation: `backtranambig -sequence protein_stats_records.fasta -outseq stdout`

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS backtranambig application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/backtranambig.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `ambiguous_backtranslation`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes
