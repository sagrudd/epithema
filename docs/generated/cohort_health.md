# Cohort Health Gate

This page is generated from the shipped cohort validation report, the governance alignment report, and the current release-candidate readiness document. Review it before reordering future roadmap sweeps.

## Summary

- Shipped methods: `111`
- Compared-evidence methods: `110`
- Methods with harvested legacy provenance recorded: `111`
- Retained backlog still unshipped: `0`
- Largest retained backlog family: `none` (`0` remaining)
- Weakest evidence family: `Modernize — Rework — Primer and assay-oriented search` (`1` methods below compared evidence)
- Release-truth document current: `no`

## Reprioritization Signals

- `weak_evidence_burden` / `notice`: 'Modernize — Rework — Primer and assay-oriented search' carries the largest weak-evidence burden 'Modernize — Rework — Primer and assay-oriented search' has 1 shipped methods below compared evidence and 2 already compared.
- `release_truth_lag` / `warning`: release-truth documentation is behind the current generated state The RC readiness document is missing current markers for: - Shipped methods audited: `111`, - Executable-evidence methods: `1`, - Methods with harvested legacy provenance recorded: `111`.

## Ordered Recommendations

1. `release readiness truth`: Refresh the RC readiness material before adding more shipped scope so release-facing documentation does not lag the generated cohort state. (`release_truth_lag`)
2. `Modernize — Rework — Primer and assay-oriented search`: This family has 1 shipped methods still below compared evidence, so it is the strongest candidate for the next acceptance/harvest deepening sweep. (`weak_evidence_burden`)
