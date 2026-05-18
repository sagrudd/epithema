# aaindexextract

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report one governed built-in amino-acid property table

## Document Metadata

- Document ID: `aaindexextract-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_stats`
- Legacy names: `aaindexextract`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/aaindexextract.validation.json`](../validation/aaindexextract.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`aaindexextract` exposes a governed built-in subset of amino-acid property indices. The EMBOSS-RS v1 implementation is intentionally narrower than historical EMBOSS and does not claim full AAINDEX database coverage. It emits one deterministic residue table for a requested built-in index.

## Inputs

The current interface accepts one required built-in index name and no sequence input. Supported index names are `hydropathy_kyte_doolittle`, `average_mass`, `charge_class`, and `polarity_class`. Short aliases `hydropathy`, `mass`, `charge`, and `polarity` are also accepted.

## Outputs

The tool emits a stable table report with columns `index`, `residue`, `three_letter`, `name`, `value`, `units`, and `notes`. Rows preserve a fixed canonical amino-acid order from `A` through `V`.

## Subset Model

Hydropathy scores use the same Kyte-Doolittle table used by `pepwindow`. Average masses use the same residue-mass table used by `pepstats`. `charge_class` and `polarity_class` are reported as coarse categorical classes rather than as historical AAINDEX numeric scales.

## Current Status

This method is implemented and exposed through `emboss-rs aaindexextract`. Validation currently covers supported built-in index parsing, stable residue-table emission, and service-level reporting through the shared statistics path.

## Caveats

The first release does not ingest external AAINDEX files or expose the broader historical index catalogue. It is a governed built-in subset only.

## Declared Artifacts

### AAindexextract hydropathy case note

- Artifact ID: `hydropathy_case`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-testkit/tests/fixtures/autodoc/aaindexextract_hydropathy_subset_case.md`
- Notes: Repository-managed case note describing the governed built-in hydropathy subset example.

## Declared Examples

### Report the built-in hydropathy subset

- Example ID: `report_hydropathy_subset`
- Description: Emits the stable amino-acid property table for the governed hydropathy subset.
- Referenced artifacts: `hydropathy_case`
- Parameters:
  - `index` = `hydropathy`
- Expected outputs:
  - `aaindexextract_table`: Built-in amino-acid property table (A stable residue table for the requested built-in index.)
- Legacy reference: EMBOSS aaindexextract application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/aaindexextract.acd`
  - Invocation: `aaindexextract -index hydropathy`

## Provenance

- Curated by: OpenAI Codex
- Source references:
  - EMBOSS aaindexextract application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/aaindexextract.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `report_hydropathy_subset`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
