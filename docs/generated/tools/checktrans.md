# checktrans

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Strictly compare frame-1 coding-sequence translations against expected protein records

## Document Metadata

- Document ID: `checktrans-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `translation_tools`
- Legacy names: `checktrans`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/checktrans.validation.json`](../validation/checktrans.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`checktrans` strictly translates DNA coding-sequence records in frame 1 under the standard genetic code and compares them against paired expected protein records. EMBOSS-RS v1 keeps the pairing rule simple and deterministic: inputs must contain the same number of records and are compared by input order.

## Inputs

The interface accepts one nucleotide coding-sequence input and one protein input. Nucleotide records must translate cleanly in frame 1 with coding-sequence length divisible by three. Protein records may end with a single terminal `*`, and one terminal stop is normalized on both sides for comparison.

## Outputs

The tool emits a stable table report with one row per paired comparison. Report columns are `nucleotide_id`, `protein_id`, `matches`, `translated_terminal_stop`, `expected_terminal_stop`, and `detail`.

## Legacy Context

This acceptance anchor keeps one historical-style `checktrans` invocation in view and compares the EMBOSS-RS table payload against a committed expected output. The governed comparison validates strict frame-1 translation, terminal-stop normalization, and deterministic row ordering.

## Current Status

This method is implemented and exposed through `emboss-rs checktrans`. Current validation covers the matching committed fixture pair, mismatch reporting, unequal record-count failure, and invalid-codon rejection.

## Caveats

The first release does not search alternative frames or infer record pairing by identifier. Inputs are compared in order only, and ambiguous or invalid coding input fails clearly rather than being tolerated.

## Declared Artifacts

### Translation-check nucleotide fixture

- Artifact ID: `checktrans_nucleotide_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/checktrans_nucleotide.fasta`
- Notes: Repository-managed coding-sequence fixture used for strict translation checking.

### Translation-check protein fixture

- Artifact ID: `checktrans_protein_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/checktrans_protein.fasta`
- Notes: Repository-managed expected protein fixture paired by input order with the committed coding sequences.

## Declared Examples

### Compare matching coding and protein records

- Example ID: `compare_matching_translation_pair`
- Description: Strictly translates the committed coding-sequence fixture in frame 1 and compares the result against the paired committed protein fixture.
- Referenced artifacts: `checktrans_nucleotide_fixture`, `checktrans_protein_fixture`
- Expected outputs:
  - `translation_check_table`: Translation comparison table (A stable row-per-pair report describing whether each strict frame-1 translation matched the expected protein record.)
- Legacy reference: EMBOSS checktrans application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/checktrans.acd`
  - Invocation: `checktrans -sequence checktrans_nucleotide.fasta -translation checktrans_protein.fasta -stdout yes`

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS checktrans application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/checktrans.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `compare_matching_translation_pair`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes
