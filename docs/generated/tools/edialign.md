# edialign

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

derive an exact shared local block across two or more sequences as a bounded local multiple alignment

## Document Metadata

- Document ID: `edialign-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `alignment_tools`
- Legacy names: `edialign`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/edialign.validation.json`](../validation/edialign.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`edialign` derives a bounded local multiple alignment from the longest exact block shared by every sequence in one input file. Epithema v1 is intentionally conservative: it emits only the shared exact block as a Stockholm alignment and does not attempt the broader historical dynamic-programming surface.

## Inputs

The current interface accepts one local sequence input containing at least two records plus an optional `--min-length` threshold. All records must belong to one compatible molecule class, and the tool searches for exact shared ungapped blocks only.

## Outputs

The output is a stable Stockholm alignment whose rows contain the exact shared block only. The result summary reports the chosen block and its source spans, while the alignment payload preserves input row order.

## Current Status

This method is implemented and exposed through `epithema edialign`. Validation currently covers one committed three-record nucleotide fixture that shares an exact four-residue block.

## Caveats

The v1 method is an exact shared-block aligner, not a full historical replacement for dynamic local multiple alignment. It does not introduce gaps, score alternative local blocks, or emit richer alignment-edit traces.

## Declared Artifacts

### Edialign FASTA fixture

- Artifact ID: `edialign_records_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/edialign_records.fasta`
- Notes: Repository-managed three-record FASTA fixture with one exact shared local block across all records.

## Declared Examples

### Derive one shared exact local block across three records

- Example ID: `derive_shared_exact_block_alignment`
- Description: Runs `edialign` against a committed FASTA fixture and emits one multiple alignment whose rows are the longest exact block shared by all three records.
- Referenced artifacts: `edialign_records_fasta`
- Expected outputs:
  - `shared_block_alignment`: Shared exact-block alignment (A three-row Stockholm alignment is emitted with the shared block `TACG` as aligned content in each row.)
- Legacy reference: EMBOSS edialign application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/edialign.acd`
  - Invocation: `edialign -sequences edialign_records.fasta -outseq stdout`

## Provenance

- Curated by: epithema maintainers
- Source references:
  - EMBOSS edialign application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/edialign.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `derive_shared_exact_block_alignment`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
