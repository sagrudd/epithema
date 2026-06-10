# biosed

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Replace or delete explicit sequence intervals record by record

## Document Metadata

- Document ID: `biosed-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_edit`
- Legacy names: `biosed`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/biosed.validation.json`](../validation/biosed.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`biosed` performs bounded interval editing over each sequence record in one local input. The Epithema v1 surface supports 1-based inclusive deletion or replacement of the same interval across all records and emits a cleaned FASTA stream rather than free-form text.

## Inputs

The current interface accepts one local sequence input plus explicit `start` and `end` coordinates. Supplying `--replace <residues>` inserts replacement residues; omitting the flag deletes the selected interval. Coordinates must remain in range for every record.

## Outputs

The tool emits edited FASTA records in input order. CLI summaries report the source input, edited interval, replacement or delete mode, coordinate convention, and the annotation-drop caveat.

## Current Status

This method is implemented and exposed through `epithema biosed`. Validation currently covers deterministic interval replacement and deletion against committed FASTA fixtures in the Rust tool and service layers.

## Caveats

The v1 implementation applies the same coordinates to every record, drops feature annotations after editing, and does not support regex-style edits, multiple independent edit operations in one invocation, or annotation-coordinate remapping.

## Declared Artifacts

### Biosed FASTA fixture

- Artifact ID: `biosed_records_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/biosed_records.fasta`
- Notes: Repository-managed FASTA input fixture used for deterministic interval editing validation.

## Declared Examples

### Replace a shared interval in every input record

- Example ID: `replace_interval_across_records`
- Description: Runs `biosed` against the committed FASTA fixture and replaces residues 2..3 with `NN` in every record.
- Referenced artifacts: `biosed_records_fasta`
- Expected outputs:
  - `biosed_output_records`: Edited sequence records (The output records are emitted as FASTA with residues `ANNG` and `TNNN`, and the shared result summary reports `Start: 2`, `End: 3`, and `Replacement: NN`.)
- Legacy reference: EMBOSS biosed application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/biosed.acd`
  - Invocation: `biosed -sequence biosed_records.fasta -start 2 -end 3 -replace NN -outseq stdout`

## Provenance

- Curated by: epithema maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `replace_interval_across_records`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
