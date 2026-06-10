Bounded provider-discovery case for `whichdb`.

- Provider: `ncbi`
- Qualified input: `ncbi:protein:NP_000537.3`
- Expected normalized query: `protein:NP_000537.3`
- Expected route label: `ncbi.reference-sequence-discovery`
- Expected discovery status: `supported_provider`
- Expected next methods: `refseqget`
- Evidence boundary: deterministic governed route reporting in Rust analytical
  coverage, not a live NCBI lookup, payload retrieval, or database-universe
  search
