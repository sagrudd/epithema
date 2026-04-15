# needle

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Global pairwise sequence alignment

## Document Metadata

- Document ID: `needle-minimal`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `alignment`
- Legacy names: `needle`

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

## Validation Intent

- Required examples: `basic_alignment`
- Compare against legacy: no
- Require provenance capture: yes

