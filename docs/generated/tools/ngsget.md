# ngsget

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Plan and materialize public ENA or SRA NGS assets for one study, sample, experiment, or run accession

## Document Metadata

- Document ID: `ngsget-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `archive_tools`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/ngsget.validation.json`](../validation/ngsget.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`ngsget` is the acquisition companion to `ngslist`. It resolves a public ENA or SRA NGS study, sample, experiment, or run accession to a normalized run-level manifest, selects generated FASTQ assets by default, optionally adds raw or submitted assets alongside any available FASTQ when `--raw` is present, materializes files under the documented run layout, and writes NGS provenance JSON.

## Inputs

The command interface is `epithema ngsget <accession> [--provider auto|ena|sra] [--out <dir>] [--raw] [--check-downloads <path>]`. Provider-qualified accessions such as `ena:PRJNA1011899` and `sra:SRR123456` are part of the contract. The optional `--check-downloads` path recursively searches an existing download tree for same-name files before network retrieval, copies verified matches into the output tree, leaves originals intact, and reports failed materialization records for same-name candidates with unexpected checksums. Custom container selection is not exposed; SRA FASTQ conversion uses the pinned default SRA Toolkit container recorded in provenance.

## Outputs

The documented output layout is `<out>/manifest.tsv`, `<out>/provenance.json`, and `runs/<run_accession>/` subdirectories for `fastq`, `raw`, `sra`, and logs. Implemented service records capture selected and skipped assets, expected and observed byte counts and MD5 checksums, direct-download verification state, SRA Toolkit conversion command details, container image and tool version metadata, exit status, and generated FASTQ output paths. Direct provider downloads stream to `.partial` files, verify from disk, and emit CLI progress events so raw archives in the tens or hundreds of gigabytes are not buffered in memory. The service can write a stable handoff `manifest.tsv` for later object-store importers without uploading or publishing objects.

## Current Status

`ngsget` is exposed through the governed command route. The implementation supports download planning, direct ENA-style materialization with streamed partial-file handling, disk-based verification, and CLI progress reporting, recursive existing-download lookup through `--check-downloads`, copy-then-verify reuse of matching files without modifying the originals, SRA archive download plus pinned-container FASTQ conversion through an injectable runner, deterministic failure/resume semantics, provenance JSON writing, stable handoff manifest TSV writing, and mocked ENA/SRA validation fixtures. Remaining follow-up work includes per-run log files, custom container selection, and opt-in live-provider or Docker/SRA Toolkit validation.

## Caveats

The first `ngsget` implementation is limited to public ENA/SRA datasets. Protected-access, dbGaP-controlled, credentialed, requester-pays, and object-store publication workflows are not implemented and must remain explicit future work. Routine validation is mocked and deterministic; live-provider or Docker/SRA Toolkit checks should stay opt-in gated checks.

## Declared Artifacts

### Mocked service NGS ingestion case fixture

- Artifact ID: `ngsget_service_ingestion_case`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-testkit/tests/fixtures/autodoc/ngsget_service_ingestion_case.md`
- Notes: Repository-managed case note covering the mocked service-layer NGS planning, materialization, SRA conversion, and provenance serialization tests.

## Declared Examples

### Materialize generated FASTQ assets for a public NGS accession

- Example ID: `materialize_public_ngs_fastq_assets`
- Description: Plans generated FASTQ acquisition for a public ENA or SRA accession and records materialized-file provenance through mocked service seams.
- Referenced artifacts: `ngsget_service_ingestion_case`
- Expected outputs:
  - `ngs_provenance_json`: NGS provenance JSON (A stable `epithema.ngs-provenance/v1` JSON document describing query metadata, selected and skipped assets, materialization records, verification evidence, and generated FASTQ paths.)

## Provenance

- Curated by: epithema maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `materialize_public_ngs_fastq_assets`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
