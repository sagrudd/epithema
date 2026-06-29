Provider-backed mocked study manifest case for `ngslist`.

- Service coverage: `executes_ngslist_against_mocked_ena_study_manifest`
- Provider route: ENA Portal API `read_run` file-report mock
- Query: `PRJNA1011899` with `--provider ena`
- Expected behavior: normalize multiple run rows into the documented one-asset-per-row table without downloading files
