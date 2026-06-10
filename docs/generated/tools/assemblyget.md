# assemblyget

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report bounded assembly-level manifest intent for one provider-qualified archive accession

## Document Metadata

- Document ID: `assemblyget-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `archive_tools`
- Legacy names: `assemblyget`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/assemblyget.validation.json`](../validation/assemblyget.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`assemblyget` is the assembly-level manifest-intent companion to `infoassembly`. In v1 it resolves one provider-qualified archive accession through the governed archive-provider seam and projects the result into a deterministic table that describes what would be considered for acquisition without acquiring it.

## Inputs

The shipped governed route accepts exactly one provider-qualified archive accession such as `ena:ERR123456` or `sra:SRR123456`. Local file paths, inline literals, bare identifiers, unqualified database names, and multi-accession batches are rejected before any provider metadata is materialized.

## Outputs

The output is a stable table with provider, requested accession, normalized object class, selected assembly or study/project accession, linked run accession when present, route endpoint, manifest mode, file count, total known bytes, and materialization status. The shipped v1 mode is explicitly `manifest_intent_only` with `not_materialized` status.

## Legacy Context

The historical EMBOSS `assemblyget` name implies acquisition of assembly-associated data. This governed Rust surface deliberately keeps only the modern provider-aware manifest-intent user need in scope. The committed compared example proves the bounded report shape against a checked-in expected table payload rather than claiming historical download or assembly-materialization parity.

## Current Status

This method is implemented and exposed through `epithema assemblyget`. Rust service coverage exercises the governed mocked ENA route and local-file rejection, and the acceptance-anchor harness compares the mocked ENA manifest-intent table against a committed expected TSV fixture.

## Caveats

`assemblyget` does not download, stage, unpack, index, write, or otherwise materialize archive files. It does not claim broad provider parity, live provider search, external database preparation, generic archive orchestration, or historical EMBOSS acquisition behaviour beyond the explicitly shipped manifest-intent report.

## Declared Artifacts

### Mocked ENA assemblyget manifest-intent case fixture

- Artifact ID: `assemblyget_mocked_ena_manifest_intent_case`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-testkit/tests/fixtures/autodoc/assemblyget_ena_err123456_case.md`
- Notes: Repository-managed case note for the mocked ENA manifest-intent route used in Rust service coverage and the acceptance-anchor comparison.

## Declared Examples

### Report ENA assembly-level manifest intent without materializing files

- Example ID: `report_ena_assembly_manifest_intent`
- Description: Resolves a provider-qualified ENA archive accession and returns the bounded manifest-intent table through the governed archive reporting path.
- Referenced artifacts: `assemblyget_mocked_ena_manifest_intent_case`
- Expected outputs:
  - `ena_assembly_manifest_intent_table`: Bounded ENA assembly manifest-intent table (A stable table report that includes the provider, requested accession, normalized object class, selected study/project accession, linked run accession, route endpoint, manifest-intent mode, file count, total known bytes, and explicit no-materialization status.)
- Legacy reference: EMBOSS assemblyget application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/assemblyget.acd`
  - Invocation: `assemblyget -sequence ena:ERR123456 -stdout yes`

## Provenance

- Curated by: epithema maintainers
- Source references:
  - EMBOSS assemblyget application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/assemblyget.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `report_ena_assembly_manifest_intent`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
