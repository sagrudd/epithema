# seqret

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Sequence format conversion

## Document Metadata

- Document ID: `seqret-rich`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `mixed`
- Tool family: `sequence_io`
- Legacy names: `seqret`

## Overview

Seqret converts sequence data between supported representations.

## Legacy context

This contract preserves example structure harvested from historical EMBOSS material.

## Caveats

Historical examples may reference obsolete output formatting.

## Declared Artifacts

### Historical EMBOSS example page

- Artifact ID: `legacy_example_html`
- Origin: legacy EMBOSS reference (EMBOSS seqret HTML example)
- Acquisition: legacy harvest
- Reference: path `legacy/emboss/seqret/example.html`
- Notes: Harvested reference artifact from the historical EMBOSS repository.

### ENA accession example

- Artifact ID: `ena_accession`
- Origin: accessioned resource
- Acquisition: provider (sequence input; preferred ena)
- Reference: accession `AB000263`
- Notes: Demonstrates provider-backed sequence acquisition.

### Generated preview output

- Artifact ID: `generated_preview`
- Origin: generated artifact
- Acquisition: generated
- Reference: generated locator `build/autodoc/seqret/preview.txt`
- Notes: Placeholder for generated preview text.

## Declared Examples

### Convert an accession-backed sequence to FASTA

- Example ID: `convert_accession_to_fasta`
- Description: Illustrates provider-backed acquisition through the governed autodoc path.
- Referenced artifacts: `ena_accession`, `generated_preview`
- Parameters:
  - `output_format` = `fasta`
- Expected outputs:
  - `fasta_output`: Converted FASTA output (Future emitted FASTA result.)
- Legacy reference: historical seqret example
  - Locator: `https://github.com/kimrutherford/EMBOSS`
  - Invocation: `seqret -sequence AB000263 -outseq out.fa`

### Normalize a harvested legacy example

- Example ID: `legacy_example_normalization`
- Description: Captures the relationship between harvested documentation and future generated output.
- Referenced artifacts: `legacy_example_html`, `generated_preview`
- Expected outputs:
  - `normalized_doc_block`: Normalized documentation block (Structured text derived from the harvested artifact.)
- Legacy reference: historical EMBOSS documentation
  - Locator: `legacy/emboss/seqret/example.html`

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS GitHub repository (`https://github.com/kimrutherford/EMBOSS`)

## Validation Intent

- Required examples: `convert_accession_to_fasta`, `legacy_example_normalization`
- Compare against legacy: yes
- Require provenance capture: yes

