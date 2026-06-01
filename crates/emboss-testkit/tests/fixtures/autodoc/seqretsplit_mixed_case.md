Bounded provider-qualified split-output case for `seqretsplit`.

- Local-input shipped example: `three_records.fasta`
- Provider-qualified exercised route: `ena:AB000263`
- Expected bounded behavior: one normalized output partition per resolved record,
  with deterministic file naming derived from the same computation path
- Service coverage: `invokes_seqretsplit_with_provider_input_into_partitioned_payload`
