Bounded mixed-input retrieval case for `seqretsetall`.

- Inputs: one local multi-record FASTA fixture plus one provider-qualified accession
- Provider: `ena`
- Qualified input: `ena:AB000263`
- Service coverage: `invokes_seqretsetall_with_mixed_inputs_into_partitioned_payload`
- Expected output shape: `ResultPayload::SequencePartitions` with one ordered record set per resolved input
- Evidence boundary: mocked provider seam in Rust service tests, not harvested live-provider acceptance evidence
