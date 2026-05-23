# featcopy

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Copy selected feature annotations from annotated source records onto compatible target records

## Document Metadata

- Document ID: `featcopy-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `feature_tools`
- Legacy names: `featcopy`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/featcopy.validation.json`](../validation/featcopy.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`featcopy` copies selected features from annotated source records onto matching target records using the shared EMBOSS-RS feature-selection and feature-copy helpers. In the current v1 behavior the underlying target sequence is preserved, existing target features are retained, and copied source features are appended in stable source order.

## Inputs

The tool accepts two local inputs: an annotated source sequence set and a target sequence set. Supported selectors are `--kind`, `--name`, `--qualifier`, and `--strand`. When more than one selector is supplied they are combined conjunctively. If no selector flags are provided, all source features are considered for copying.

## Outputs

The output preserves target record order and emits the target sequences in FASTA form while retaining copied annotations in the structured payload. Records are paired by stable accession identifier, both inputs must contain the same identifier set, and paired record lengths must match before any features are copied.

## Copy Semantics

Selected source features are cloned with their names, notes, qualifiers, and locations preserved exactly and then appended to the corresponding target record. The v1 tool does not rewrite sequence residues or rebase coordinates, and it does not attempt to merge or deduplicate copied features against any pre-existing target annotations.

## Current Status

This method is implemented and exposed through `emboss-rs featcopy`. Validation currently covers copying a selected gene feature from a committed annotated GenBank fixture onto a matching plain FASTA target. Rust service tests also cover qualifier-based copying with qualifier preservation and explicit identifier mismatch or no-match failures.

## Caveats

The current v1 output mode is conservative: copy selected features onto otherwise preserved target records. More elaborate feature-table export or annotation-diff workflows are deferred. Inputs with mismatched identifier sets or record lengths fail explicitly instead of being partially paired.

## Declared Artifacts

### Annotated GenBank fixture

- Artifact ID: `annotated_feature_genbank`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/annotated_feature.gbk`
- Notes: Repository-managed annotated GenBank fixture used as the deterministic source for featcopy validation.

### Matching FASTA target fixture

- Artifact ID: `featcopy_target_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/featcopy_target.fasta`
- Notes: Repository-managed FASTA target fixture with the same accession and length as the annotated source.

## Declared Examples

### Copy a selected gene feature onto a matching target record

- Example ID: `copy_selected_gene_feature_to_matching_target`
- Description: Selects the `gene` feature from the annotated source fixture and appends it to the equal-length target record with the same accession.
- Referenced artifacts: `annotated_feature_genbank`, `featcopy_target_fasta`
- Parameters:
  - `kind` = `gene`
- Expected outputs:
  - `feature_copied_sequences`: Feature-copied target sequences (The target sequence is returned unchanged at the residue level with the selected source feature appended to its annotations.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `copy_selected_gene_feature_to_matching_target`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
