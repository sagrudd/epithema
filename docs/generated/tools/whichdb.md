# whichdb

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report bounded provider-discovery routes for one provider-qualified query

## Document Metadata

- Document ID: `whichdb-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `retrieval_tools`
- Legacy names: `whichdb`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/whichdb.validation.json`](../validation/whichdb.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`whichdb` reports deterministic provider-discovery rows for one provider-qualified accession or identifier. The EMBOSS-RS v1 surface normalizes the provider prefix and provider-local query, then reports the governed retrieval or metadata methods that can be used next.

## Inputs

The current interface accepts exactly one provider-qualified query such as `ena:AB000263`. Bare identifiers, local file paths, inline sequence literals, and unqualified database names are rejected because this seam does not perform database-universe discovery.

## Outputs

The result is a stable table report with provider, normalized query, route label, discovery status, and next governed methods. Supported providers currently return bounded routing guidance; unsupported providers are reported explicitly without fallback expansion.

## Current Status

This method is implemented and exposed through `emboss-rs whichdb`. Rust service coverage exercises the governed service route for a supported ENA query and verifies that unsupported providers are reported without fallback.

## Caveats

The v1 `whichdb` seam is intentionally bounded. It does not perform live provider search, payload retrieval, archive download, local file indexing, broad database discovery, or a fallback chain across provider universes. Canonical fixtures and compared evidence are tracked as follow-up work.

## Declared Artifacts

### Bounded ENA whichdb route fixture

- Artifact ID: `whichdb_bounded_ena_case`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-testkit/tests/fixtures/autodoc/whichdb_ena_ab000263_case.md`
- Notes: Repository-managed case note for the bounded ENA provider-discovery route exercised by Rust service coverage.

## Declared Examples

### Report the bounded ENA provider-discovery route

- Example ID: `report_ena_provider_discovery_route`
- Description: Normalizes an ENA-qualified query and reports the governed retrieval or metadata methods available for that provider-qualified route.
- Referenced artifacts: `whichdb_bounded_ena_case`
- Parameters:
  - `query` = `ena:AB000263`
- Expected outputs:
  - `whichdb_table`: Bounded whichdb route table (A stable table row with provider `ena`, normalized query `AB000263`, route label `ena.sequence-or-archive-discovery`, discovery status `supported_provider`, and next methods `seqret,runinfo,runget,infoassembly`.)
- Legacy reference: EMBOSS whichdb application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/whichdb.acd`
  - Invocation: `whichdb -showall N`

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS whichdb application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/whichdb.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `report_ena_provider_discovery_route`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes
