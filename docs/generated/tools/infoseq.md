# infoseq

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report stable basic metadata and length summaries for sequence records

## Document Metadata

- Document ID: `infoseq-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_stats`
- Legacy names: `infoseq`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/infoseq.validation.json`](../validation/infoseq.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`infoseq` reports one stable summary row per input record. The Epithema v1 surface is intentionally narrower than historical EMBOSS formatting switches: it keeps one governed table shape with identifier, optional display name, length, molecule, alphabet, optional GC percentage for nucleotide-like inputs, feature count, and selected descriptive metadata.

## Inputs

The current interface accepts one local sequence input path. Inputs are loaded through the shared Epithema readers for FASTA, FASTQ, EMBL, and GenBank. Plain sequence records and richer annotated records are both supported.

## Outputs

The tool emits a stable table report with columns `ordinal`, `identifier`, `display_name`, `length`, `molecule`, `alphabet`, `gc_percent`, `feature_count`, `description`, and `organism`. GC percentage is reported only for nucleotide-like records with at least one canonical nucleotide symbol; other records emit `-` in the rendered table.

## Statistics Model

Rows preserve source record order. Length is the normalized residue length after shared sequence parsing. GC percentage reuses the governed canonical-symbol model already used by `geecee`: only canonical A/C/G/T/U symbols contribute to the denominator, and non-nucleotide or non-canonical-only records omit the value.

## Current Status

This method is implemented and exposed through `epithema infoseq`. Validation currently covers plain FASTA metadata reporting, annotation-aware feature counts and organism metadata, and stable per-record table emission through the shared service layer.

## Caveats

The first release does not reproduce the large historical EMBOSS column-selection surface such as `-only`, `-heading`, or delimiter switching. It provides one governed table schema only.

## Declared Artifacts

### Three-record FASTA fixture

- Artifact ID: `three_record_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/three_records.fasta`
- Notes: Repository-managed multi-record FASTA fixture used to validate stable basic metadata reporting.

## Declared Examples

### Report basic sequence information

- Example ID: `report_basic_sequence_information`
- Description: Loads a committed multi-record FASTA fixture and emits one stable summary row per record.
- Referenced artifacts: `three_record_fasta`
- Expected outputs:
  - `infoseq_table`: Sequence information table (A stable tabular report with identifier, length, molecule, alphabet, GC percentage, and selected descriptive metadata.)
- Legacy reference: EMBOSS infoseq application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/infoseq.acd`
  - Invocation: `infoseq -sequence three_records.fasta -stdout yes`

## Provenance

- Curated by: OpenAI Codex
- Source references:
  - EMBOSS infoseq application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/infoseq.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `report_basic_sequence_information`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
