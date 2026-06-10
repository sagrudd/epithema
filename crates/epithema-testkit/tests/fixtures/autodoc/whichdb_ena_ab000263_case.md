Bounded provider-discovery case for `whichdb`.

- Provider: `ena`
- Qualified input: `ena:AB000263`
- Service coverage: `dispatches_whichdb_through_the_governed_service_surface`
- Expected route label: `ena.sequence-or-archive-discovery`
- Expected discovery status: `supported_provider`
- Expected next methods: `seqret,runinfo,runget,infoassembly`
- Evidence boundary: deterministic governed route reporting in Rust service
  tests, not live provider search, payload retrieval, archive download, local
  file indexing, or database-universe discovery
