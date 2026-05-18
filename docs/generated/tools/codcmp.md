# codcmp

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Compare codon usage between two coding-sequence or codon-profile sources

## Document Metadata

- Document ID: `codcmp-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `codon_tools`
- Legacy names: `codcmp`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/codcmp.validation.json`](../validation/codcmp.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`codcmp` compares normalized codon usage between two inputs and reports codon-by-codon count and frequency differences. In v1 both sides are normalized into the shared 61-sense-codon profile model before comparison.

## Inputs

The current interface accepts two local inputs. Each side may be either a strict coding-sequence file or a normalized codon-profile TSV produced by `codcopy`.

## Outputs

The result is a stable table report over the 61 sense codons. Each row records the codon, amino acid, left and right counts, left and right frequencies, and the frequency delta. The shared result summary also includes total variation distance across the compared profiles.

## Current Status

This method is implemented and exposed through `emboss-rs codcmp`. Rust service coverage compares two committed coding fixtures and verifies representative codon count differences in the emitted table.

## Caveats

The first release is intentionally narrow and local-only. Coding inputs are subject to the same strict validation rules as the rest of the codon-analysis cohort, and only normalized TSV profiles with the governed header format are accepted as profile inputs.

## Declared Artifacts

### Left codon-reference FASTA fixture

- Artifact ID: `codon_reference_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/codon_reference.fasta`
- Notes: Repository-managed left-hand coding fixture with preferred leucine codons.

### Right codon-compare FASTA fixture

- Artifact ID: `codon_compare_right_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/codon_compare_right.fasta`
- Notes: Repository-managed right-hand coding fixture with rarer leucine codon usage for comparison.

## Declared Examples

### Compare codon usage between two committed coding fixtures

- Example ID: `compare_two_coding_fixtures`
- Description: Normalizes two committed coding FASTA fixtures and reports codon-by-codon differences plus aggregate distance.
- Referenced artifacts: `codon_reference_fasta`, `codon_compare_right_fasta`
- Expected outputs:
  - `codon_comparison_table`: Codon comparison table (A stable comparison table in which the `CTT` row shows `5` counts on the left and `0` counts on the right for the committed fixtures.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `compare_two_coding_fixtures`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes

