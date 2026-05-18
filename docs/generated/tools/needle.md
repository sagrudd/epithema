# needle

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Global pairwise sequence alignment

## Document Metadata

- Document ID: `needle-minimal`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `pairwise_alignment`
- Legacy names: `needle`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/needle.validation.json`](../validation/needle.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

Needle computes a global alignment between two sequences.

## Declared Artifacts

### Example FASTA input

- Artifact ID: `example_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `fixtures/needle/example_fasta`
- Notes: Repository-managed fixture for a minimal example.

## Declared Examples

### Basic alignment example

- Example ID: `basic_alignment`
- Description: Demonstrates a single alignment run.
- Referenced artifacts: `example_fasta`
- Parameters:
  - `gapopen` = `10`
- Expected outputs:
  - `report`: Alignment report (Placeholder for the future rendered report.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `basic_alignment`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes

