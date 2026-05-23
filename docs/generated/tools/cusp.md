# cusp

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Create complete per-record and aggregate codon usage tables from coding sequences

## Document Metadata

- Document ID: `cusp-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `codon_tools`
- Legacy names: `cusp`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/cusp.validation.json`](../validation/cusp.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`cusp` creates codon-usage tables from coding nucleotide sequences. The EMBOSS-RS v1 surface uses the shared strict coding-sequence validation already established for the codon-analysis family and emits one complete 61-sense-codon table per record plus one aggregate table across all records.

## Inputs

The current interface accepts one local nucleotide input path. Inputs are loaded through the shared sequence readers and validated as strict in-frame coding sequences. One terminal stop codon is allowed and excluded from the profile; internal stops, ambiguous codons, and non-triplet lengths are rejected.

## Outputs

The tool emits a stable table report with columns `scope`, `record`, `codon`, `amino_acid`, `count`, `frequency`, and `terminal_stop`. Unlike the lighter `chips` report, `cusp` emits all 61 sense codons for every record and for the aggregate summary, including zero-count rows.

## Counting Model

Codons are normalized to DNA space before counting. Frequencies are reported over sense codons only. Terminal stop codons are tracked separately for per-record rows and never contribute to the codon counts or frequencies.

## Current Status

This method is implemented and exposed through `emboss-rs cusp`. Validation currently covers strict coding-sequence acceptance, complete per-record and aggregate 61-codon table emission, and stable service-level reporting against committed coding fixtures.

## Caveats

The first release does not implement alternative genetic codes or richer codon-bias statistics beyond counts and frequencies. It uses the standard genetic code only.

## Declared Artifacts

### Coding reference FASTA fixture

- Artifact ID: `codon_reference_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/codon_reference.fasta`
- Notes: Repository-managed coding-sequence fixture used to validate deterministic codon-table generation.

## Declared Examples

### Report a complete codon-usage table

- Example ID: `report_complete_codon_usage_table`
- Description: Loads a committed coding-sequence fixture and emits one complete 61-sense-codon table per record plus one aggregate table.
- Referenced artifacts: `codon_reference_fixture`
- Expected outputs:
  - `cusp_table`: Codon usage table (A stable tabular report containing complete per-record and aggregate sense-codon rows, including zero-count codons.)
- Legacy reference: EMBOSS cusp application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/cusp.acd`
  - Invocation: `cusp -sequence codon_reference.fasta -outfile stdout`

## Provenance

- Curated by: OpenAI Codex
- Source references:
  - EMBOSS cusp application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/cusp.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `report_complete_codon_usage_table`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
