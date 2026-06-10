# iep

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Estimate deterministic protein isoelectric points from a fixed explicit pKa model

## Document Metadata

- Document ID: `iep-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_stats`
- Legacy names: `iep`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/iep.validation.json`](../validation/iep.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`iep` estimates one protein isoelectric point per input record. The Epithema v1 implementation uses a fixed explicit pKa model for the N terminus, C terminus, and the D/E/C/Y/H/K/R side chains, and reports the same model's net charge at pH 7.0.

## Inputs

The current interface accepts one local protein input path. Inputs are loaded through the shared Epithema readers for FASTA, FASTQ, EMBL, and GenBank. Explicit nucleotide inputs are rejected. Gap symbols are ignored; stop symbols are ignored before estimation; unsupported ambiguous protein residues are rejected.

## Outputs

The tool emits a stable table report with columns `record`, `residue_length`, `titratable_side_chains`, `aspartate`, `glutamate`, `cysteine`, `tyrosine`, `histidine`, `lysine`, `arginine`, `net_charge_ph7`, and `estimated_pi`.

## Model

This first release is intentionally narrow and deterministic. It solves for the pI by bisection over a fixed pH range using the shared explicit pKa set and Henderson-Hasselbalch style charge fractions. It does not expose alternative pKa models, pH titration curves, or solvent-condition tuning.

## Current Status

This method is implemented and exposed through `epithema iep`. Validation currently covers basic versus mixed protein fixtures, stable per-record reporting, and nucleotide-input rejection.

## Caveats

The first release is a governed deterministic estimate, not a broad biochemical modeling surface. Ambiguous residues such as `X` or `Z` cause the run to fail instead of being approximated.

## Declared Artifacts

### Protein pI fixture

- Artifact ID: `iep_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/iep_records.fasta`
- Notes: Repository-managed protein fixture used to validate deterministic per-record pI estimation.

## Declared Examples

### Estimate pI for protein records

- Example ID: `estimate_pi_for_protein_records`
- Description: Loads a small protein fixture and reports one deterministic pI estimate per record.
- Referenced artifacts: `iep_fixture`
- Expected outputs:
  - `iep_table`: Protein pI table (A stable tabular report containing per-record titratable counts, net charge at pH 7.0, and estimated pI.)
- Legacy reference: EMBOSS iep application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/iep.acd`
  - Invocation: `iep -sequence iep_records.fasta -outfile stdout`

## Provenance

- Curated by: OpenAI Codex
- Source references:
  - EMBOSS iep application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/iep.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `estimate_pi_for_protein_records`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
