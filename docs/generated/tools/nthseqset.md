# nthseqset

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Select one alignment set by 1-based ordinal position from a multi-alignment input

## Document Metadata

- Document ID: `nthseqset-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `alignment_tools`
- Legacy names: `nthseqset`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/nthseqset.validation.json`](../validation/nthseqset.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`nthseqset` selects one alignment set from an input that may contain more than one multiple alignment. EMBOSS-RS v1 supports deterministic 1-based selection from Stockholm files containing multiple alignments and also accepts a single aligned FASTA or Stockholm input as set `1`.

## Inputs

The current interface accepts one local alignment input path plus one selection number. The selection number is **1-based** and must refer to an existing alignment set after the shared loader has parsed the input.

## Outputs

The tool emits the selected alignment through the shared alignment result path. CLI summaries report the input path, selected set number, total set count, and Stockholm output format.

## Current Status

This method is implemented and exposed through `emboss-rs nthseqset`. Validation currently covers deterministic selection of the second alignment from a committed multi-Stockholm fixture, single-alignment passthrough as set `1`, and out-of-range failure behavior.

## Caveats

The first release supports ordinal set selection only. It does not support identifier-based alignment selection, combined reporting across sets, or grouped filtering within one invocation.

## Declared Artifacts

### Two-alignment Stockholm fixture

- Artifact ID: `nthseqset_stockholm_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/nthseqset_alignments.sto`
- Notes: Repository-managed Stockholm fixture containing two distinct alignment sets for deterministic ordinal selection.

## Declared Examples

### Select the second alignment set from a multi-Stockholm input

- Example ID: `select_second_alignment_set`
- Description: Runs `nthseqset` against a committed Stockholm fixture containing two alignments and returns only the second set.
- Referenced artifacts: `nthseqset_stockholm_fixture`
- Parameters:
  - `number` = `2`
- Expected outputs:
  - `selected_alignment_set`: Selected alignment as Stockholm (A Stockholm alignment containing the second parsed set, whose first row accession is `gamma`.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `select_second_alignment_set`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
