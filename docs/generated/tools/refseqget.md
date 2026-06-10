# refseqget

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Retrieve one provider-qualified reference sequence accession through the governed provider seam

## Document Metadata

- Document ID: `refseqget-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `retrieval_tools`
- Legacy names: `refseqget`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/refseqget.validation.json`](../validation/refseqget.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`refseqget` is the accession-only retrieval counterpart to `seqret`. It resolves one provider-qualified reference sequence accession through the governed sequence-provider seam and returns a single normalized sequence record.

## Inputs

The current v1 interface accepts exactly one provider-qualified accession, for example `ncbi:protein:NP_000537.3`. Local file inputs are rejected deliberately, and inline literal sequence text is not accepted by the service path.

## Outputs

The result is one normalized sequence record with retrieval provenance attached to the shared record metadata. The method is intended for accession-backed reference retrieval, not general file normalization.

## Current Status

This method is implemented and exposed through `epithema refseqget`. The accession-backed retrieval seam is covered in Rust service tests with a mocked NCBI FASTA response, and local-file rejection is also tested explicitly.

## Caveats

The current curated evidence is seam-oriented rather than live-network acceptance evidence. `refseqget` does not currently accept local inputs, free-text sequence literals, or loosely specified bare accessions, and the first release intentionally keeps the provider contract narrower than historical EMBOSS breadth.

## Declared Artifacts

### Mocked NCBI retrieval case fixture

- Artifact ID: `refseqget_mocked_provider_case`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-testkit/tests/fixtures/autodoc/refseqget_ncbi_np_000537_3_case.md`
- Notes: Repository-managed case note for the mocked NCBI provider-qualified retrieval used in Rust service coverage.

## Declared Examples

### Retrieve a provider-qualified reference sequence accession

- Example ID: `retrieve_provider_qualified_reference_sequence`
- Description: Resolves an explicitly qualified NCBI protein accession through the governed provider seam and returns one normalized sequence record.
- Referenced artifacts: `refseqget_mocked_provider_case`
- Expected outputs:
  - `retrieved_reference_sequence`: Retrieved reference sequence (A single normalized sequence record carrying the expected accession and `ncbi` source provenance.)

## Provenance

- Curated by: epithema maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `retrieve_provider_qualified_reference_sequence`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes
