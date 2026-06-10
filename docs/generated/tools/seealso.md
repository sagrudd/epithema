# seealso

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report deterministic bounded related-program rows from governed local tool metadata

## Document Metadata

- Document ID: `seealso-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `command_tools`
- Legacy names: `seealso`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/seealso.validation.json`](../validation/seealso.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`seealso` is the bounded related-program member of the active command discovery and help-navigation rework program. The Epithema v1 surface resolves one governed local tool and reports deterministic related-program rows derived from governed family metadata and bounded short-description term overlap.

## Inputs

The current interface accepts exactly one governed tool name. The bounded seam resolves that tool against the local governed registry and searches only governed tool family and short-description metadata. Live provider lookup, ontology expansion, and broad semantic ranking remain out of scope.

## Outputs

The result is a stable normalized table with query tool identity, related tool identity, related governed family, related governed short description, normalized relationship terms, and explicit relationship-bearing metadata fields. Rows are deterministic and derive only from the governed local catalog.

## Legacy Context

This bounded release keeps one historical `seealso` user need in scope while modernizing around deterministic local governed-metadata lookup instead of broad semantic ranking, ontology expansion, synonym-taxonomy inference, or generalized help-navigation behavior.

## Current Status

This method is implemented and exposed through `epithema seealso`. The governed surface ships with a checked-in validation stub, curated legacy provenance, Rust coverage for the bounded local related-program lookup path, and canonical compared evidence for deterministic related-program rows.

## Caveats

The v1 `seealso` seam is intentionally narrow. It does not perform broad semantic ranking, ontology expansion, provider-backed discovery, or generalized command-discovery behavior, and it does not widen the command-discovery family merely because this bounded related-program slice ships.

## Declared Artifacts

### Seealso query-tool fixture

- Artifact ID: `seealso_query_tool_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/seealso_query_tool.txt`
- Notes: Repository-managed query-tool fixture used to anchor deterministic bounded seealso validation.

## Declared Examples

### Report deterministic related-program rows for a governed local tool

- Example ID: `seealso_related_programs_example`
- Description: Executes the bounded `seealso` seam against the governed local tool catalog with `needle` as the query tool to emit stable normalized related-program rows.
- Referenced artifacts: `seealso_query_tool_fixture`
- Parameters:
  - `tool_name` = `needle`
- Expected outputs:
  - `seealso_table`: Bounded seealso analytical table (Stable normalized related-program rows with query tool, related tool, governed family, governed short description, relationship terms, and relationship fields.)
- Legacy reference: EMBOSS seealso application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/seealso.acd`
  - Invocation: `seealso -program needle -outfile stdout`

## Provenance

- Curated by: epithema maintainers
- Source references:
  - EMBOSS seealso application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/seealso.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `seealso_related_programs_example`
- Future legacy comparison requested: yes
- Future execution must capture provenance: yes
