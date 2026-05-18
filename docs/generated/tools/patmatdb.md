# patmatdb

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

scan protein sequences with a local deterministic motif database

## Document Metadata

- Document ID: `patmatdb-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `pattern_tools`
- Legacy names: `patmatdb`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/patmatdb.validation.json`](../validation/patmatdb.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`patmatdb` searches protein sequence records against a local motif database and reports stable motif hits in a table. EMBOSS-RS v1 keeps the database contract deliberately narrow and local rather than recreating historical database plumbing.

## Inputs

The current interface accepts one local protein input plus one local TSV motif database. Each non-comment database row must contain `<motif-id><TAB><pattern><TAB><optional-description>`. Pattern syntax matches `fuzzpro`: exact amino-acid residues plus `X` as a single-residue wildcard.

## Outputs

The output is a stable hit table with one row per motif match. Each row reports the source record, motif identifier, pattern text, optional description, 1-based inclusive coordinates, and the matched sequence slice.

## Current Status

This method is implemented and exposed through `emboss-rs patmatdb`. Validation currently covers a committed two-motif TSV database and a committed protein FASTA fixture through the Rust tool and service layers.

## Caveats

The first release supports only local TSV motif databases and the bounded `fuzzpro` pattern model. It does not integrate with external motif archives, profile libraries, or richer scoring semantics.

## Declared Artifacts

### Patmatdb protein FASTA fixture

- Artifact ID: `patmatdb_records_fasta`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/patmatdb_records.fasta`
- Notes: Repository-managed protein FASTA fixture used for motif-database searching.

### Patmatdb motif TSV fixture

- Artifact ID: `patmatdb_motif_tsv`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/patmatdb_motifs.tsv`
- Notes: Repository-managed TSV motif database fixture containing two simple protein motifs.

## Declared Examples

### Report hits from a local motif database

- Example ID: `report_hits_from_local_motif_database`
- Description: Runs `patmatdb` against a committed protein FASTA fixture and a committed TSV motif database.
- Referenced artifacts: `patmatdb_records_fasta`, `patmatdb_motif_tsv`
- Expected outputs:
  - `patmatdb_hit_table`: Motif database hit table (Two hits are reported from the first fixture record, one for each motif entry in the committed TSV database.)
- Legacy reference: EMBOSS patmatdb application
  - Locator: `https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/patmatdb.acd`
  - Invocation: `patmatdb -sequence patmatdb_records.fasta -full patmatdb_motifs.tsv -outfile stdout`

## Provenance

- Curated by: emboss-rs maintainers
- Source references:
  - EMBOSS patmatdb application (`https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/patmatdb.acd`)

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `report_hits_from_local_motif_database`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes

