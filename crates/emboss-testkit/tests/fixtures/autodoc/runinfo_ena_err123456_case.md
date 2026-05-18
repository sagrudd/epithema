Provider-backed mocked metadata case for `runinfo`.

- Provider: `ena`
- Qualified input: `ena:ERR123456`
- Service coverage: `executes_runinfo_against_mocked_ena_run_metadata`
- Expected output shape: normalized table report with FASTQ-oriented rows and provider summary `Provider: ena`
- Evidence boundary: mocked provider seam in Rust service tests, not harvested live-provider acceptance evidence
