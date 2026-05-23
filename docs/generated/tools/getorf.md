# getorf

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Extract stop-bounded forward ORFs from nucleotide records

## Document Metadata

- Document ID: `getorf-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `translation_tools`
- Legacy names: `getorf`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/getorf.validation.json`](../validation/getorf.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`getorf` extracts stop-bounded open reading frames from nucleotide input using the shared EMBOSS-RS translation foundations. The current v1 policy searches only forward frames 1-3, starts ORFs at `ATG`, and ends each ORF at the first in-frame stop codon, including the stop codon in the extracted nucleotide output.

## Inputs

The tool accepts one local nucleotide input path. Protein-classified input is rejected. Ambiguous or unsupported codons are not expanded in the first release.

## Outputs

The tool emits one nucleotide FASTA record per detected ORF in stable record and frame order. Output identifiers are suffixed as `<accession>.orfN.frameF.<start>-<end>`, and descriptions are annotated with the reported frame and coordinates.

## Legacy Context

This acceptance anchor keeps one historical-style `getorf` invocation in view and compares the EMBOSS-RS FASTA payload against a committed expected output. The governed comparison validates the first-release stop-bounded forward-ORF policy on the committed nucleotide fixture.

## Current Status

This method is implemented and exposed through `emboss-rs getorf`. Current validation covers ORF extraction from the committed stop-bounded fixture, stable coordinate encoding in identifiers, and clear rejection of non-nucleotide input.

## Caveats

The first release supports forward frames only, requires `ATG` starts, and stops at the first in-frame stop codon. It does not expose alternative start codons, minimum-length filtering, reverse-complement search, or alternative genetic codes.

## Declared Artifacts

### Stop-bounded ORF fixture

- Artifact ID: `getorf_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/getorf_records.fasta`
- Notes: Repository-managed nucleotide fixture containing two forward stop-bounded ORFs for deterministic ORF extraction validation.

## Declared Examples

### Extract forward stop-bounded ORFs

- Example ID: `extract_stop_bounded_forward_orfs`
- Description: Scans the committed nucleotide fixture for `ATG`-started forward ORFs and emits each stop-bounded nucleotide span as FASTA.
- Referenced artifacts: `getorf_fixture`
- Expected outputs:
  - `orf_fasta_records`: Extracted ORF FASTA output (A stable FASTA rendering of the extracted stop-bounded forward ORFs.)
- Legacy reference: EMBOSS getorf application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/getorf.acd`
  - Invocation: `getorf -sequence getorf_records.fasta -outseq stdout`

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS getorf application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/getorf.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `extract_stop_bounded_forward_orfs`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes
