Provider-backed mocked metadata case for `infoassembly`.

- Provider: `ena`
- Qualified input: `ena:ERR123456`
- Service coverage: `invokes_infoassembly_with_mocked_ena_metadata_into_table_payload`
- Expected output shape: bounded `field`/`value` report with assembly accession, file count, total known bytes, and provider route summary
- Evidence boundary: mocked provider seam in Rust service tests, not harvested live-provider acceptance evidence
