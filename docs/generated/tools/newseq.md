# newseq

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Create a new sequence record from inline residues and typed metadata

## Document Metadata

- Document ID: `newseq-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_stream`
- Legacy names: `newseq`

## Overview

`newseq` constructs one new sequence record from user-supplied inline residues and shared EMBOSS-RS metadata fields. The record is validated through the core sequence model and emitted through the standard FASTA output path instead of being assembled as ad hoc text.

## Inputs

The current v1 interface requires an explicit record identifier and inline sequence content. Optional flags allow a free-text description and an explicit molecule kind chosen from `dna`, `rna`, `protein`, or `unknown`.

## Outputs

The tool emits one validated sequence record through the shared sequence output layer. CLI output is FASTA plus the standard EMBOSS-RS method summary lines reporting identifier, length, molecule, alphabet, and description state.

## Current Status

This method is implemented and exposed through `emboss-rs newseq`. Validation currently covers explicit DNA creation, explicit protein creation, whitespace normalization, and invalid residue rejection for declared molecule kinds.

## Caveats

The v1 molecule policy is intentionally conservative. When `--molecule` is omitted, EMBOSS-RS infers DNA or RNA only from unambiguous nucleotide residue sets and otherwise falls back to `unknown` rather than guessing protein. Identifier omission is not supported in v1.

## Declared Artifacts

No artifacts are declared for this autodoc document.

## Declared Examples

### Create a DNA record from inline residues

- Example ID: `create_dna_record`
- Description: Builds one DNA sequence record with an explicit identifier, description, and declared molecule kind.
- Referenced artifacts: none declared
- Parameters:
  - `identifier` = `created`
  - `sequence` = `ACGTAC`
  - `description` = `created example`
  - `molecule` = `dna`
- Expected outputs:
  - `created_sequence_record`: Created sequence record (A normalized DNA record is emitted as FASTA and reported through the shared method result summary.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Validation Intent

- Required examples: `create_dna_record`
- Compare against legacy: no
- Require provenance capture: yes

