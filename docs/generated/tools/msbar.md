# msbar

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Apply explicit point mutations to sequence records

## Document Metadata

- Document ID: `msbar-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_edit`
- Legacy names: `msbar`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/msbar.validation.json`](../validation/msbar.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`msbar` applies one or more explicit point substitutions to every sequence record in one local input. The EMBOSS-RS v1 surface is bounded to deterministic substitution-only editing using `position:residue` syntax.

## Inputs

The current interface accepts one local sequence input followed by one or more mutation specifications written as `position:residue`. Positions are 1-based, must be unique within a single invocation, and are applied to every input record.

## Outputs

The tool emits mutated FASTA records in input order. CLI summaries report the source input, mutation count, mutation syntax, and FASTA output format.

## Current Status

This method is implemented and exposed through `emboss-rs msbar`. Validation currently covers deterministic substitution behavior against committed FASTA fixtures in the Rust tool and service layers.

## Caveats

The v1 implementation supports substitutions only. It does not support insertions, deletions, ambiguous edit grammars, or annotation-coordinate remapping, and it drops feature annotations after mutation.

## Declared Artifacts

### msbar FASTA fixture

- Artifact ID: `msbar_records_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/msbar_records.fasta`
- Notes: Repository-managed FASTA input fixture used for deterministic point-mutation validation.

## Declared Examples

### Apply two point mutations to one input record

- Example ID: `apply_point_mutations`
- Description: Runs `msbar` against the committed FASTA fixture and applies mutations `2:T` and `4:A`.
- Referenced artifacts: `msbar_records_fasta`
- Expected outputs:
  - `mutated_sequence_record`: Mutated sequence record (The output FASTA record contains residues `ATGA`, and the shared result summary reports that two mutations were applied using `position:residue` syntax.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `apply_point_mutations`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
