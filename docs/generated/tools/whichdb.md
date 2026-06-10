# whichdb

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report bounded provider-discovery routes for one provider-qualified query

## Document Metadata

- Document ID: `whichdb-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `retrieval_tools`
- Legacy names: `whichdb`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/whichdb.validation.json`](../validation/whichdb.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`whichdb` reports deterministic provider-discovery rows for one provider-qualified accession or identifier. The Epithema v1 surface normalizes the provider prefix and provider-local query, then reports the governed retrieval or metadata methods that can be used next. The method is a route-reporting seam: it tells the caller which shipped method family can handle a bounded provider route, rather than retrieving data itself.

## Inputs

The current interface accepts exactly one provider-qualified query such as `ena:AB000263`, `ncbi:protein:NP_000537.3`, or `sra:SRR000001`. The provider prefix is lower-cased; the provider-local query is trimmed and otherwise preserved, including nested provider-local database qualifiers after the first colon. Bare identifiers, local file paths, inline sequence literals, and unqualified database names are rejected because this seam does not perform database-universe discovery.

## Outputs

The result is a stable table report with provider, normalized query, route label, discovery status, and next governed methods. Supported ENA, NCBI, and SRA provider routes return bounded routing guidance. Unsupported provider prefixes are still reported as syntactically valid rows with `unsupported_provider` status and an empty next-method list, so callers can distinguish unsupported scope from parse failures.

## Current Status

This method is implemented and exposed through `epithema whichdb`. Rust service coverage exercises the governed service route for a supported ENA query and verifies that unsupported providers are reported without fallback.

## Caveats

The v1 `whichdb` seam is intentionally bounded. It does not perform live provider search, payload retrieval, archive download, local file indexing, broad database discovery, or a fallback chain across provider universes. Unsupported-provider rows document that the provider is outside the shipped bounded surface; they are not retrieval promises.

## Declared Artifacts

### Bounded ENA whichdb route fixture

- Artifact ID: `whichdb_bounded_ena_case`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-testkit/tests/fixtures/autodoc/whichdb_ena_ab000263_case.md`
- Notes: Repository-managed case note for the bounded ENA provider-discovery route exercised by Rust service coverage.

### Bounded NCBI whichdb route fixture

- Artifact ID: `whichdb_bounded_ncbi_case`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-testkit/tests/fixtures/autodoc/whichdb_ncbi_nested_reference_case.md`
- Notes: Repository-managed case note for preserving nested NCBI provider-local database qualifiers in bounded route reporting.

### Unsupported-provider whichdb route fixture

- Artifact ID: `whichdb_unsupported_provider_case`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-testkit/tests/fixtures/autodoc/whichdb_unsupported_provider_case.md`
- Notes: Repository-managed case note for explicit unsupported-provider reporting without fallback expansion.

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

### Report the bounded NCBI reference-sequence route

- Example ID: `report_ncbi_nested_reference_route`
- Description: Normalizes an NCBI-qualified query while preserving the nested provider-local database qualifier after the first colon.
- Referenced artifacts: `whichdb_bounded_ncbi_case`
- Parameters:
  - `query` = `ncbi:protein:NP_000537.3`
- Expected outputs:
  - `whichdb_ncbi_table`: Bounded NCBI whichdb route table (A stable table row with provider `ncbi`, normalized query `protein:NP_000537.3`, route label `ncbi.reference-sequence-discovery`, discovery status `supported_provider`, and next method `refseqget`.)
- Legacy reference: EMBOSS whichdb application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/whichdb.acd`
  - Invocation: `whichdb -showall N`

### Report unsupported providers without fallback expansion

- Example ID: `report_unsupported_provider_without_fallback`
- Description: Reports a syntactically valid but unsupported provider-qualified query as an explicit no-fallback row.
- Referenced artifacts: `whichdb_unsupported_provider_case`
- Parameters:
  - `query` = `uniprot:P12345`
- Expected outputs:
  - `whichdb_unsupported_provider_table`: Unsupported-provider whichdb route table (A stable table row with provider `uniprot`, normalized query `P12345`, route label `unsupported-provider`, discovery status `unsupported_provider`, and no next methods.)
- Legacy reference: EMBOSS whichdb application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/whichdb.acd`
  - Invocation: `whichdb -showall N`

## Provenance

- Curated by: epithema maintainers
- Source references:
  - EMBOSS whichdb application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/whichdb.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `report_ena_provider_discovery_route`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes
