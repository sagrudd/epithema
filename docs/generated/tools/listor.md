# listor

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Apply deterministic exact set operations to two sequence inputs

## Document Metadata

- Document ID: `listor-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_stream`
- Legacy names: `listor`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/listor.validation.json`](../validation/listor.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`listor` performs a bounded logical set operation across two local sequence inputs. EMBOSS-RS v1 treats sequence sets as exact normalized sequence-content sets keyed by molecule kind and uppercase residues, removes duplicates within each input before the operator is applied, and preserves first-seen representatives in deterministic output order.

## Inputs

The current interface accepts two local sequence inputs and an optional operator flag. Supported operators are `OR`, `AND`, `XOR`, and `NOT`, with `OR` as the default. Each input may contain one or more sequence records.

## Outputs

The tool emits a sequence collection through the shared FASTA output path rather than a historical list-file projection. CLI summaries report the input paths, selected operator, duplicate counts removed from each side before set logic, and the final returned record count.

## Current Status

This method is implemented and exposed through `emboss-rs listor`. Validation currently covers deterministic `OR`, `NOT`, and duplicate-elimination behavior in the Rust tool layer, plus a governed service example for `XOR` against committed fixtures.

## Caveats

The first release does not implement identifier-based set logic, fuzzy sequence matching, or a historical USA-list output mode. Duplicate handling is exact and sequence-based, not accession-based.

## Declared Artifacts

### First set FASTA fixture

- Artifact ID: `listor_first_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/listor_first.fasta`
- Notes: Repository-managed first input fixture containing two records for deterministic logical set validation.

### Second set FASTA fixture

- Artifact ID: `listor_second_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/listor_second.fasta`
- Notes: Repository-managed second input fixture sharing one exact sequence with the first fixture and contributing one unique sequence.

## Declared Examples

### Return only sequence records unique to each side

- Example ID: `xor_two_sequence_sets`
- Description: Runs `listor` against two committed FASTA fixtures and applies the `XOR` operator after exact duplicate elimination within each input.
- Referenced artifacts: `listor_first_fasta`, `listor_second_fasta`
- Parameters:
  - `operator` = `XOR`
- Expected outputs:
  - `xor_sequence_collection`: Unique sequence representatives (A FASTA sequence collection containing the `beta` and `delta` representatives in stable first-seen order.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `xor_two_sequence_sets`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
