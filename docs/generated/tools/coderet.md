# coderet

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Extract selected simple coding features and optionally translate them

## Document Metadata

- Document ID: `coderet-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `feature_tools`
- Legacy names: `coderet`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/coderet.validation.json`](../validation/coderet.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`coderet` extracts simple coding-feature regions from annotated EMBL or GenBank records using the shared Epithema feature-selection and extraction seam. In v1 it defaults to CDS selection when no selector flags are supplied, and can optionally translate each extracted region with the standard genetic code.

## Inputs

The current v1 tool accepts one annotated EMBL or GenBank input plus optional `--translate`, `--kind`, `--name`, `--qualifier`, and `--strand` flags. Feature selectors combine conjunctively when more than one is supplied. If no selector flags are present, `coderet` defaults conservatively to `--kind cds`.

## Outputs

Without `--translate`, one nucleotide FASTA record is emitted per selected simple feature in stable source and feature order. With `--translate`, each extracted region is translated strictly in frame 1 and emitted as a protein FASTA record with a `.pep` identifier suffix.

## Current Status

This method is implemented and exposed through `epithema coderet`. Rust tests currently cover default CDS extraction, strict translation, no-match handling through the shared feature-extraction seam, and service-layer invocation against committed annotated fixtures.

## Caveats

The v1 scope supports only simple single-span feature locations. Joined or otherwise complex locations fail clearly through the shared feature-extraction layer. Translation is strict and does not attempt alternative codes, frame inference, or tolerant CDS repair.

## Declared Artifacts

### Annotated GenBank fixture

- Artifact ID: `annotated_feature_genbank`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/annotated_feature.gbk`
- Notes: Repository-managed annotated GenBank fixture used for deterministic coderet validation.

## Declared Examples

### Translate the default CDS selection

- Example ID: `translate_default_cds_selection`
- Description: Uses the default CDS selector and translates the extracted coding region into a protein FASTA record.
- Referenced artifacts: `annotated_feature_genbank`
- Parameters:
  - `translate` = `true`
- Expected outputs:
  - `coderet_sequences`: Coding-feature-derived sequences (One translated protein record is emitted for the CDS feature in the fixture.)
- Legacy reference: EMBOSS coderet application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/coderet.acd`
  - Invocation: `coderet -sequence annotated_feature.gbk -translate -outseq stdout`

## Provenance

- Curated by: epithema maintainers
- Source references:
  - EMBOSS coderet application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/coderet.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `translate_default_cds_selection`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
