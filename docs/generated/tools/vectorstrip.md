# vectorstrip

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Strip exact vector sequences from the ends of nucleotide records

## Document Metadata

- Document ID: `vectorstrip-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_edit`
- Legacy names: `vectorstrip`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/vectorstrip.validation.json`](../validation/vectorstrip.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`vectorstrip` removes an exact vector sequence from the left and right ends of each nucleotide input record. The EMBOSS-RS v1 implementation is intentionally conservative and strips only full-length exact terminal matches from one committed vector record.

## Inputs

The current interface accepts one local nucleotide sequence input and one local vector input containing exactly one vector record. The tool requires nucleotide-like content and matching or compatible nucleotide molecule kinds.

## Outputs

The tool emits vector-stripped FASTA records in input order. CLI summaries report the source input, vector input, exact terminal match rule, and FASTA output format.

## Current Status

This method is implemented and exposed through `emboss-rs vectorstrip`. Validation currently covers deterministic left-end, right-end, and unchanged-record behavior against committed FASTA fixtures in the Rust tool and service layers.

## Caveats

The v1 implementation strips only exact full-length terminal matches, does not search reverse complements, does not perform approximate vector matching, and drops feature annotations after editing.

## Declared Artifacts

### vectorstrip FASTA fixture

- Artifact ID: `vectorstrip_records_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/vectorstrip_records.fasta`
- Notes: Repository-managed FASTA input fixture containing records with left, right, and absent exact vector matches.

### vectorstrip vector FASTA fixture

- Artifact ID: `vectorstrip_vector_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/vectorstrip_vector.fasta`
- Notes: Repository-managed single-record vector FASTA fixture used for deterministic exact terminal stripping validation.

## Declared Examples

### Strip exact vector matches from sequence ends

- Example ID: `strip_exact_terminal_vector_matches`
- Description: Runs `vectorstrip` against committed input and vector FASTA fixtures and removes full-length exact terminal vector matches from both sequence ends where present.
- Referenced artifacts: `vectorstrip_records_fasta`, `vectorstrip_vector_fasta`
- Expected outputs:
  - `vectorstrip_output_records`: Vector-stripped sequence records (The output FASTA records contain residues `ACGT`, `TTAA`, and `GGCC`, showing exact stripping from both ends, one end, and no-change cases respectively.)
- Legacy reference: EMBOSS vectorstrip application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/vectorstrip.acd`
  - Invocation: `vectorstrip -sequence vectorstrip_records.fasta -vector vectorstrip_vector.fasta -outseq stdout`

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `strip_exact_terminal_vector_matches`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
