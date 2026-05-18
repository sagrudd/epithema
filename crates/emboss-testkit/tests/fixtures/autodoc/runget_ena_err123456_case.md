Provider-backed mocked manifest case for `runget`.

- Provider: `ena`
- Qualified input: `ena:ERR123456`
- Service coverage: `executes_runget_against_mocked_ena_manifest`
- Expected output shape: normalized manifest table without downloading files
- Evidence boundary: mocked provider seam in Rust service tests, not harvested live-provider acceptance evidence
