# wossname

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report deterministic bounded keyword matches against governed local tool metadata

## Document Metadata

- Document ID: `wossname-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `command_tools`
- Legacy names: `wossname`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/wossname.validation.json`](../validation/wossname.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`wossname` is the bounded command-discovery member of the active command discovery and help-navigation rework program. The EMBOSS-RS v1 surface searches governed local tool metadata with deterministic normalized keyword matching and returns a stable table-first report rather than a semantic-ranking or ontology-driven discovery workflow.

## Inputs

The current interface accepts exactly one free-text keyword query. The bounded seam normalizes case, tokenizes the query into local keyword terms, and searches governed tool names plus governed short descriptions only. Live provider lookup, asset-distribution behavior, and broader synonym expansion remain out of scope.

## Outputs

The result is a stable normalized table with matched tool identity, governed tool family, governed short description, matched normalized query terms, and explicit match-bearing text fields. All rows derive from the same deterministic local lookup path and preserve governed catalog order.

## Legacy Context

This bounded release keeps one historical `wossname` user need in scope while modernizing around deterministic local governed-metadata lookup instead of broad semantic ranking, ontology expansion, or generalized help-navigation behavior. The governed validation seam is shipped now and the canonical compared fixture is the next bounded follow-on slice.

## Current Status

This method is implemented and exposed through `emboss-rs wossname`. The governed surface ships with a checked-in validation stub, curated legacy provenance, and Rust coverage for the bounded local keyword-lookup path. The shipped evidence posture is currently executable-only until the canonical compared keyword-match fixture lands.

## Caveats

The v1 `wossname` seam is intentionally narrow. It does not perform broad semantic ranking, ontology expansion, synonym-taxonomy inference, or provider-backed discovery, and it does not widen into `embossdata` or `seealso` merely because this bounded command-discovery slice ships.

## Declared Artifacts

### Wossname keyword-query fixture

- Artifact ID: `wossname_query_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/wossname_query.txt`
- Notes: Repository-managed keyword query fixture used to anchor deterministic bounded wossname validation.

## Declared Examples

### Report deterministic keyword matches against governed local tool metadata

- Example ID: `wossname_keyword_matches_example`
- Description: Executes the bounded `wossname` seam against the governed local tool catalog with a committed keyword query to emit stable normalized keyword-match rows.
- Referenced artifacts: `wossname_query_fixture`
- Parameters:
  - `query` = `pairwise align`
- Expected outputs:
  - `wossname_table`: Bounded wossname analytical table (Stable normalized keyword-match rows with tool identity, governed family, governed short description, matched query terms, and matched text fields.)
- Legacy reference: EMBOSS wossname application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/wossname.acd`
  - Invocation: `wossname -search "pairwise align" -outfile stdout`

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS wossname application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/wossname.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `wossname_keyword_matches_example`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes
