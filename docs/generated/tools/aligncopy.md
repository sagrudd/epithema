# aligncopy

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Copy a single alignment unchanged through the shared alignment IO path

## Document Metadata

- Document ID: `aligncopy-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `alignment_tools`
- Legacy names: `aligncopy`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/aligncopy.validation.json`](../validation/aligncopy.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`aligncopy` is the alignment-preserving equivalent of `seqret` for the current Epithema alignment surface. It loads one aligned FASTA or Stockholm alignment through the shared alignment reader and writes the same alignment payload back through the governed alignment result path without modifying rows, columns, or metadata.

## Inputs

The current v1 interface accepts exactly one local alignment input path. Supported input formats are the alignment formats already handled by the shared alignment loader, currently aligned FASTA and Stockholm.

## Outputs

The result is the normalized alignment payload itself. Through the CLI, Epithema renders alignment outputs as Stockholm by default so the copied alignment remains structured and round-trippable.

## Current Status

This method is implemented and exposed through `epithema aligncopy`. The declared example below matches the existing Rust service test path that loads a committed multiple-alignment fixture and confirms row-count and alignment-identifier preservation.

## Caveats

The first release is intentionally narrow: `aligncopy` is a normalization and pass-through command, not an editing or reformatting workflow. Historical EMBOSS option breadth is not reproduced yet.

## Declared Artifacts

### Multiple-alignment Stockholm fixture

- Artifact ID: `multiple_alignment_stockholm`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/multiple_alignment.sto`
- Notes: Repository-managed Stockholm alignment fixture with three aligned rows and a stable alignment identifier.

## Declared Examples

### Copy a multiple Stockholm alignment unchanged

- Example ID: `copy_multiple_alignment_stockholm`
- Description: Loads the committed three-row Stockholm alignment fixture and returns the same alignment through the governed output path.
- Referenced artifacts: `multiple_alignment_stockholm`
- Expected outputs:
  - `copied_alignment`: Copied alignment payload (A three-row alignment that preserves the input alignment identifier and column layout.)

## Provenance

- Curated by: epithema maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `copy_multiple_alignment_stockholm`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
