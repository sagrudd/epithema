# backtranseq

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Back-translate protein records into deterministic representative DNA codons

## Document Metadata

- Document ID: `backtranseq-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `translation_tools`
- Legacy names: `backtranseq`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/backtranseq.validation.json`](../validation/backtranseq.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`backtranseq` back-translates protein sequence records into one deterministic representative DNA coding sequence per residue using the shared Epithema standard-code back-translation core. The v1 behavior is intentionally narrow and reproducible rather than biologically expansive.

## Inputs

The current interface accepts one local protein-oriented sequence input path. Explicit nucleotide-classified records are rejected. Records classified as `unknown` remain acceptable when the shared loaders preserve protein-like residue content.

## Outputs

The tool emits one DNA FASTA record per input protein record in stable order. Output identifiers are preserved, the output molecule kind is `dna`, and descriptions are annotated with the suffix `representative back-translated DNA`.

## Legacy Context

This acceptance anchor keeps one historical-style `backtranseq` invocation in view and compares the Epithema FASTA payload against a committed expected output. The comparison is intentionally scoped to deterministic representative codons under the standard genetic code.

## Current Status

This method is implemented and exposed through `epithema backtranseq`. Current validation covers the deterministic codon policy, stop-symbol handling with `* -> TAA`, and stable FASTA emission from committed protein fixtures.

## Caveats

The first release uses one fixed representative DNA codon per residue and does not expose codon-usage weighting, alternate genetic codes, or RNA output. Unsupported residues fail clearly instead of being approximated.

## Declared Artifacts

### Protein back-translation fixture

- Artifact ID: `protein_stats_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/protein_stats_records.fasta`
- Notes: Repository-managed protein fixture used for deterministic representative back-translation validation.

## Declared Examples

### Back-translate protein records into deterministic representative DNA

- Example ID: `representative_backtranslation`
- Description: Back-translates the committed protein fixture into one representative DNA coding sequence per residue using the governed Epithema standard-code policy.
- Referenced artifacts: `protein_stats_fixture`
- Expected outputs:
  - `representative_dna_sequences`: Representative DNA FASTA output (A stable FASTA rendering of the deterministic representative DNA sequences.)
- Legacy reference: EMBOSS backtranseq application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/backtranseq.acd`
  - Invocation: `backtranseq -sequence protein_stats_records.fasta -outseq stdout`

## Provenance

- Curated by: epithema maintainers
- Source references:
  - EMBOSS backtranseq application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/backtranseq.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `representative_backtranslation`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes
