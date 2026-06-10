# diffseq

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

compare two similar sequences and report contiguous difference blocks from a deterministic global alignment

## Document Metadata

- Document ID: `diffseq-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `alignment_tools`
- Legacy names: `diffseq`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/diffseq.validation.json`](../validation/diffseq.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`diffseq` compares exactly one sequence from each input, computes a deterministic global alignment, and reports contiguous mismatch or indel blocks in a governed table. Epithema v1 treats the alignment as the comparison substrate and emits typed block coordinates instead of a prose-only narrative.

## Inputs

The current interface accepts exactly one local sequence record from each of two inputs. Optional `--gap-open` and `--gap-extend` flags tune the deterministic global-alignment penalties. Both inputs must resolve to a compatible nucleotide or protein comparison mode.

## Outputs

The output is a stable table with one row per contiguous difference block. Each row reports a classification, optional 1-based inclusive coordinates in both sequences, and the aligned block segments. When the sequences are identical, the table is empty and the summary states that zero difference blocks were found.

## Current Status

This method is implemented and exposed through `epithema diffseq`. Validation currently covers one committed substitution case through the Rust tool and service layers.

## Caveats

The first release is a deterministic block report, not a full recreation of historical EMBOSS difference narration. It does not attempt feature-aware commentary, biological effect interpretation, or alternative comparison heuristics beyond the governed global-alignment path.

## Declared Artifacts

### Diffseq left FASTA fixture

- Artifact ID: `diffseq_left_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/diffseq_left.fasta`
- Notes: Repository-managed one-record FASTA fixture used as the left-hand diffseq input.

### Diffseq right FASTA fixture

- Artifact ID: `diffseq_right_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/diffseq_right.fasta`
- Notes: Repository-managed one-record FASTA fixture used as the right-hand diffseq input.

## Declared Examples

### Report one substitution block between two similar sequences

- Example ID: `report_single_substitution_block`
- Description: Runs `diffseq` against two committed one-record FASTA fixtures that differ at exactly one position.
- Referenced artifacts: `diffseq_left_fasta`, `diffseq_right_fasta`
- Expected outputs:
  - `difference_block_table`: Difference block table (One row is reported with a `substitution` classification and matching 1-based block coordinates at position `5`.)
- Legacy reference: EMBOSS diffseq application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/diffseq.acd`
  - Invocation: `diffseq -asequence diffseq_left.fasta -bsequence diffseq_right.fasta -outfile stdout`

## Provenance

- Curated by: epithema maintainers
- Source references:
  - EMBOSS diffseq application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/diffseq.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `report_single_substitution_block`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
