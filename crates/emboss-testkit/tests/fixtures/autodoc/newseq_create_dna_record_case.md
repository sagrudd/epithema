# `newseq` inline DNA creation case

- Tool: `newseq`
- Example ID: `create_dna_record`
- Invocation form:
  - `emboss-rs newseq created ACGTAC --description "created example" --molecule dna`
- Purpose:
  - Exercise the inline record-construction path for a simple DNA sequence.
  - Confirm that explicit identifier, description, and declared molecule kind are accepted.
  - Provide governed input provenance for the executable validation declaration.
- Expected characteristics:
  - one FASTA record is emitted
  - identifier is `created`
  - sequence normalizes to `ACGTAC`
  - molecule is reported as `dna`
