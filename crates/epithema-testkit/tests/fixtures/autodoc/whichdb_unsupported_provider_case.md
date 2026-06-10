Bounded provider-discovery case for `whichdb`.

- Provider: `uniprot`
- Qualified input: `uniprot:P12345`
- Expected normalized query: `P12345`
- Expected route label: `unsupported-provider`
- Expected discovery status: `unsupported_provider`
- Expected next methods: empty
- Evidence boundary: deterministic no-fallback reporting in Rust analytical
  coverage, not a claim that UniProt retrieval is implemented by `whichdb`
