# makeprotseq

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Create deterministic protein sequence records from a bounded random generator

## Document Metadata

- Document ID: `makeprotseq-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_stream`
- Legacy names: `makeprotseq`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/makeprotseq.validation.json`](../validation/makeprotseq.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`makeprotseq` creates one or more deterministic protein sequence records from a canonical amino-acid alphabet and a documented seed. The v1 implementation is intentionally narrow and governed, prioritizing reproducible generated records over a broad randomization surface.

## Inputs

The current interface requires an identifier prefix and an exact residue length. Optional flags control `--count`, `--seed`, and a shared description string attached to generated records.

## Outputs

The tool emits one or more FASTA protein records through the shared sequence output path. CLI summaries report the identifier prefix, length, count, deterministic seed, and output format.

## Current Status

This method is implemented and exposed through `emboss-rs makeprotseq`. Validation currently covers deterministic seed behavior, identifier-prefix behavior, and protein output typing in the Rust tool and service layers.

## Caveats

The v1 surface supports only canonical amino-acid generation with exact-length output. It does not model residue frequencies, motifs, low-complexity constraints, or biologically informed composition targets.

## Declared Artifacts

### makeprotseq governed case note

- Artifact ID: `makeprotseq_case`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-testkit/tests/fixtures/autodoc/makeprotseq_create_protein_record_case.md`
- Notes: Repository-managed note describing the deterministic protein generation case used for the governed `makeprotseq` surface.

## Declared Examples

### Generate one deterministic protein record

- Example ID: `generate_one_protein_record`
- Description: Builds one five-residue protein sequence record from a documented seed.
- Referenced artifacts: `makeprotseq_case`
- Parameters:
  - `identifier-prefix` = `made_prot`
  - `length` = `5`
  - `seed` = `9`
- Expected outputs:
  - `generated_protein_record`: Generated protein record (One FASTA protein record named `made_prot` is emitted, and the shared result summary reports the deterministic seed and protein molecule kind.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `generate_one_protein_record`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
