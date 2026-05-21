# maskambigprot

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Mask conservative protein ambiguity residues with X while preserving record order

## Document Metadata

- Document ID: `maskambigprot-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `feature_tools`
- Legacy names: `maskambigprot`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/maskambigprot.validation.json`](../validation/maskambigprot.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`maskambigprot` rewrites a conservative set of protein ambiguity symbols to `X` using the shared sequence-record model. EMBOSS-RS v1 keeps the scope explicit: only protein records are accepted, record order is preserved, and canonical amino-acid symbols remain unchanged.

## Inputs

The current tool accepts one local protein sequence input through the governed IO layer. FASTA is the primary exercised format in validation. Nucleotide inputs are rejected.

## Outputs

The tool emits a FASTA sequence collection through the shared result path. Identifiers and metadata are preserved, and attached annotations remain present in the payload because masking is performed directly on the existing residue coordinates.

## Current Status

This method is implemented and exposed through `emboss-rs maskambigprot`. Validation currently covers deterministic masking of an ambiguous protein FASTA fixture and explicit rejection of nucleotide input in the Rust tool layer.

## Caveats

The first release masks only `B`, `J`, `X`, and `Z`. It does not treat `O` or `U` as ambiguity symbols, and it leaves gap or stop markers unchanged.

## Declared Artifacts

### Ambiguous protein FASTA fixture

- Artifact ID: `ambiguous_protein_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/ambiguous_protein_records.fasta`
- Notes: Repository-managed protein FASTA fixture containing conservative ambiguity residues for deterministic masking validation.

## Declared Examples

### Mask conservative protein ambiguity symbols with X

- Example ID: `mask_protein_ambiguities`
- Description: Runs `maskambigprot` against a committed protein FASTA fixture and rewrites ambiguity residues while leaving canonical amino-acid symbols unchanged.
- Referenced artifacts: `ambiguous_protein_fasta`
- Expected outputs:
  - `masked_protein_sequences`: Ambiguity-masked protein FASTA output (Stable FASTA output where the fixture residue string `MBJZXUO-*` becomes `MXXXXUO-*`.)
- Legacy reference: EMBOSS maskambigprot application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/maskambigprot.acd`
  - Invocation: `maskambigprot -sequence ambiguous_protein_records.fasta -outseq stdout`

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `mask_protein_ambiguities`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes

