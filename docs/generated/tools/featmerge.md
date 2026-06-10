# featmerge

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Merge selected right-hand features into identifier-matched annotated records

## Document Metadata

- Document ID: `featmerge-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `feature_tools`
- Legacy names: `featmerge`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/featmerge.validation.json`](../validation/featmerge.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`featmerge` merges selected features from a right-hand annotated input into identifier-matched left-hand annotated records using the shared Epithema feature-copy and annotated-record seams. The left-hand residues and metadata are preserved, and exact duplicate features are skipped deterministically.

## Inputs

The current v1 tool accepts two annotated EMBL or GenBank inputs plus optional feature selectors. Record pairing is by exact identifier set, paired lengths must match, and selectors apply only to right-hand features before the merge.

## Outputs

One output FASTA record is emitted per left-hand record in stable input order. Left-hand features are preserved first, admitted right-hand features are appended in stable source order, and exact duplicate features are not re-added.

## Current Status

This method is implemented and exposed through `epithema featmerge`. Rust tests currently cover successful merges, deterministic duplicate skipping, no-merge failure when a selector admits only duplicates, and service invocation against committed annotated fixtures.

## Caveats

The v1 merge is structural rather than source-textual: it preserves the normalized Epithema feature model, not original flatfile formatting. Record pairing is strict, duplicate identifiers are rejected, and more elaborate overlap reconciliation is intentionally deferred.

## Declared Artifacts

### Left-hand annotated GenBank fixture

- Artifact ID: `annotated_merge_left`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/annotated_feature.gbk`
- Notes: Repository-managed left-hand annotated fixture used for feature-merge validation.

### Right-hand annotated GenBank fixture

- Artifact ID: `annotated_merge_right`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-tools/tests/fixtures/annotated_merge_right.gbk`
- Notes: Repository-managed right-hand annotated fixture containing one duplicate feature and one mergeable feature.

## Declared Examples

### Merge selected right-hand annotations into left-hand records

- Example ID: `merge_right_annotations`
- Description: Merges the right-hand annotated fixture into the left-hand record set and admits only the non-duplicate right-hand feature.
- Referenced artifacts: `annotated_merge_left`, `annotated_merge_right`
- Expected outputs:
  - `merged_feature_sequences`: Feature-merged sequences (The left-hand record is preserved and one non-duplicate feature is appended from the right-hand record.)
- Legacy reference: EMBOSS featmerge application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/featmerge.acd`
  - Invocation: `featmerge -left annotated_feature.gbk -right annotated_merge_right.gbk -outseq stdout`

## Provenance

- Curated by: epithema maintainers
- Source references:
  - EMBOSS featmerge application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/featmerge.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `merge_right_annotations`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
