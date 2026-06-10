# revseq

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Reverse sequence content and reverse-complement nucleotide records

## Document Metadata

- Document ID: `revseq-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_edit`
- Legacy names: `revseq`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/revseq.validation.json`](../validation/revseq.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`revseq` reverses each input record and can reverse-complement nucleotide sequences through the shared Epithema sequence model. The v1 implementation uses molecule-aware behavior instead of raw string munging so DNA and RNA are complemented correctly while protein records remain biologically conservative.

## Inputs

The current tool accepts local sequence inputs through the governed sequence IO path. Multi-record inputs are supported and record order is preserved. FASTA is the primary exercised format in the current validation path, with other sequence formats depending on the shared IO layer.

## Outputs

Output is a normalized sequence collection rendered through the shared result and CLI layers. By default, records classified as DNA or RNA are reverse-complemented, while protein and unknown-molecule records are reversed without complementing. `--reverse-only` forces plain reversal, and `--complement` requires nucleotide reverse-complement behavior.

## Current Status

This method is implemented and exposed through `epithema revseq`. Validation currently covers DNA auto reverse-complement behavior and reverse-only output against a committed FASTA fixture. Records with attached features are rejected in v1 because feature-coordinate remapping is not yet implemented, and conservative FASTA molecule inference means some residue-only records remain `unknown` unless a richer source format carries molecule metadata.

## Caveats

Explicit reverse-complement requests fail for protein or unknown-molecule records. Unsupported nucleotide residues also fail clearly instead of being guessed. Residue-only FASTA inputs such as short GC-only records may remain `unknown` under the current conservative inference policy and therefore use reverse-only behavior in auto mode. Richer historical examples and broader fixture coverage can be added later through the same autodoc path without changing the generated page structure.

## Declared Artifacts

### Three-record FASTA fixture

- Artifact ID: `three_record_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/three_records.fasta`
- Notes: Repository-managed FASTA fixture used for deterministic revseq validation.

## Declared Examples

### Reverse-complement nucleotide records in auto mode

- Example ID: `auto_reverse_complement_fixture`
- Description: Runs `revseq` on a small DNA FASTA fixture using the default molecule-aware behavior.
- Referenced artifacts: `three_record_fasta`
- Parameters:
  - `mode` = `auto`
- Expected outputs:
  - `auto_transformed_sequences`: Auto-mode transformed sequences (DNA records are reverse-complemented and emitted as normalized FASTA output.)
- Legacy reference: EMBOSS revseq application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/revseq.acd`
  - Invocation: `revseq -sequence three_records.fasta -outseq stdout`

## Provenance

- Curated by: epithema maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `auto_reverse_complement_fixture`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
