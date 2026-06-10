# infobase

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report deterministic metadata for one nucleotide base or ambiguity symbol

## Document Metadata

- Document ID: `infobase-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_stats`
- Legacy names: `infobase`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/infobase.validation.json`](../validation/infobase.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`infobase` reports one deterministic metadata row for a single nucleotide symbol. The Epithema v1 implementation supports canonical DNA/RNA bases plus the standard IUPAC ambiguity symbols and reports a governed classification rather than exposing multiple historical output layouts.

## Inputs

The current interface accepts exactly one nucleotide symbol. The symbol may be a canonical DNA or RNA base such as `A`, `C`, `G`, `T`, or `U`, or one of the standard IUPAC ambiguity symbols such as `R`, `Y`, `N`, or `V`.

## Outputs

The tool emits a stable one-row table with columns `symbol`, `name`, `class`, `supported_molecules`, `canonical_expansion`, `dna_complement`, and `rna_complement`.

## Symbol Model

Canonical expansion is reported as the set of underlying canonical A/C/G/T/U bases covered by the symbol. Complement columns are reported separately for DNA and RNA interpretation. Unsupported symbols fail clearly instead of being approximated.

## Current Status

This method is implemented and exposed through `epithema infobase`. Validation currently covers ambiguity-symbol lookup, stable one-row table emission, and rejection of unsupported symbols.

## Caveats

The first release is a symbol-information lookup only. It does not model modified bases, chemical formulae, or broader residue ontologies.

## Declared Artifacts

### Infobase ambiguity lookup case

- Artifact ID: `ambiguity_case`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-testkit/tests/fixtures/autodoc/infobase_lookup_ambiguity_symbol_case.md`
- Notes: Repository-managed case note describing the ambiguity-symbol lookup example.

## Declared Examples

### Look up the `N` ambiguity symbol

- Example ID: `lookup_any_base_symbol`
- Description: Reports the stable one-row metadata table for the `N` any-base symbol.
- Referenced artifacts: `ambiguity_case`
- Parameters:
  - `base` = `N`
- Expected outputs:
  - `infobase_table`: Nucleotide-symbol metadata row (A stable one-row table including canonical expansion plus DNA and RNA complements.)
- Legacy reference: EMBOSS infobase application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/infobase.acd`
  - Invocation: `infobase -base N`

## Provenance

- Curated by: OpenAI Codex
- Source references:
  - EMBOSS infobase application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/infobase.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `lookup_any_base_symbol`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
