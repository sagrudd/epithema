# infoassembly

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Normalize provider-backed archive metadata into a bounded assembly-first report

## Document Metadata

- Document ID: `infoassembly-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `archive_tools`
- Legacy names: `infoassembly`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/infoassembly.validation.json`](../validation/infoassembly.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`infoassembly` is the assembly-first archive metadata companion to `runinfo`. In v1 it resolves one provider-qualified archive accession through the governed archive-provider seam and projects normalized archive metadata into a bounded assembly-first report.

## Inputs

The current interface accepts exactly one provider-qualified archive accession such as `ena:ERR123456` or `sra:SRR123456`. Local file inputs and inline literals are rejected deliberately because `infoassembly` is a provider-backed metadata lookup tool.

## Outputs

The result is a stable `field`/`value` table report plus provider-summary lines. The bounded v1 surface reports the selected assembly identifier, linked archive identifiers, library metadata when present, file counts, total known bytes, and the normalized provider route label.

## Legacy Context

This bounded release keeps one historical `infoassembly` user need in scope while modernizing around deterministic normalized assembly-first metadata instead of broad archive-schema or acquisition behavior. The committed compared example stays narrow and provider-aware: one mocked ENA accession is resolved through the governed archive seam and compared against a checked-in expected table payload.

## Current Status

This method is implemented and exposed through `emboss-rs infoassembly`. The bounded mocked ENA assembly-first path is compared against a committed expected table fixture through the acceptance-anchor harness, and Rust service coverage also exercises mocked ENA and mocked SRA metadata paths through the same governed archive-provider seam. That proves the shipped retrieval surface without claiming broad provider parity or hidden live-network validation.

## Caveats

The v1 surface is intentionally conservative. `infoassembly` does not accept local files, free-text metadata blobs, or hidden live-network fallback behavior. It projects assembly-first metadata from the current normalized archive seam and does not yet claim broad provider-parity or full assembly-schema completeness.

## Declared Artifacts

### Mocked ENA infoassembly case fixture

- Artifact ID: `infoassembly_mocked_ena_case`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-testkit/tests/fixtures/autodoc/infoassembly_ena_err123456_case.md`
- Notes: Repository-managed case note for the mocked ENA assembly-first metadata path used in Rust service coverage.

## Declared Examples

### Normalize ENA archive metadata into a bounded assembly-first report

- Example ID: `normalize_ena_assembly_metadata`
- Description: Resolves a provider-qualified ENA archive accession and returns the current governed assembly-first metadata table and provider summary.
- Referenced artifacts: `infoassembly_mocked_ena_case`
- Expected outputs:
  - `ena_infoassembly_table`: Normalized ENA assembly-first metadata table (A stable `field`/`value` report that includes the selected assembly accession, linked archive identifiers, file counts, total known bytes, and the provider route label.)
- Legacy reference: EMBOSS infoassembly application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/infoassembly.acd`
  - Invocation: `infoassembly -sequence ena:ERR123456 -stdout yes`

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS infoassembly application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/infoassembly.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `normalize_ena_assembly_metadata`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes
