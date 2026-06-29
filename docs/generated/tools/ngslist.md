# ngslist

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

List public ENA or SRA NGS assets for one study, sample, experiment, or run accession

## Document Metadata

- Document ID: `ngslist-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `archive_tools`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/ngslist.validation.json`](../validation/ngslist.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`ngslist` expands a public ENA or SRA NGS study, sample, experiment, or run accession into a normalized run-level manifest and reports the associated generated FASTQ, provider-native SRA, submitted raw, submitted alignment, index, or unclassified submitted assets.

## Inputs

The current interface accepts one accession plus optional provider and format flags: `epithema ngslist <accession> [--provider auto|ena|sra] [--format table|json]`. Provider-qualified accessions such as `ena:PRJNA1011899` and `sra:SRR123456` are accepted. Bare or `auto` queries use the deterministic service route documented in the NGS ingestion plan.

## Outputs

The default output is a stable table with one row per asset and columns for provider, query accession, resolved object class, study/sample/experiment/run metadata, sequencing metadata, asset role, asset format, source URL, size bytes, and MD5 checksum. `--format json` returns the same asset rows as deterministic JSON text.

## Current Status

This method is implemented and exposed through `epithema ngslist`. Rust service coverage includes a mocked ENA study query that expands to multiple runs, a mocked ENA run-level JSON rendering path, provider selection parsing, and service-level provider policy enforcement through the NGS retrieval gateway.

## Caveats

`ngslist` is a public manifest listing surface only. It does not download files, convert SRA archives to FASTQ, write provenance files, handle protected-access datasets, or publish object-store records. Downloading, conversion, and provenance writing belong to the planned `ngsget` surface; protected-access, dbGaP-controlled, credentialed, and object-store publication workflows remain explicit future work.

## Declared Artifacts

### Mocked ENA ngslist study case fixture

- Artifact ID: `ngslist_mocked_ena_study_case`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-testkit/tests/fixtures/autodoc/ngslist_ena_prjna1011899_case.md`
- Notes: Repository-managed case note for the mocked ENA study manifest expansion path used in Rust service coverage.

## Declared Examples

### List assets for an ENA study accession

- Example ID: `list_ena_study_assets`
- Description: Resolves a public ENA study accession and returns the normalized run-level asset table without downloading files.
- Referenced artifacts: `ngslist_mocked_ena_study_case`
- Expected outputs:
  - `ena_study_asset_table`: Normalized ENA study asset table (A stable table report with one row per generated FASTQ or submitted NGS asset exposed by the mocked ENA study manifest.)

## Provenance

- Curated by: epithema maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `list_ena_study_assets`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
