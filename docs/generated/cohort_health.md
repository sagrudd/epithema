# Cohort Health Gate

This page is generated from the shipped cohort validation report, the governance alignment report, and the current release-candidate readiness document. Review it before reordering future roadmap sweeps.

## Summary

- Shipped methods: `112`
- Compared-evidence methods: `111`
- Methods with harvested legacy provenance recorded: `112`
- Retained backlog still unshipped: `0`
- Largest retained backlog family: `none` (`0` remaining)
- Weakest evidence family: `Modernize — Rework — Command discovery and help-navigation` (`1` methods below compared evidence)
- Release-truth document current: `no`

## Reprioritization Signals

- `weak_evidence_burden` / `notice`: 'Modernize — Rework — Command discovery and help-navigation' carries the largest weak-evidence burden 'Modernize — Rework — Command discovery and help-navigation' has 1 shipped methods below compared evidence and 0 already compared.
- `release_truth_lag` / `warning`: release-truth documentation is behind the current generated state The RC readiness document is missing current markers for: - Shipped methods audited: `112`, - Executable-evidence methods: `1`, - Methods with harvested legacy provenance recorded: `112`.

## Ordered Recommendations

1. `release readiness truth`: Refresh the RC readiness material before adding more shipped scope so release-facing documentation does not lag the generated cohort state. (`release_truth_lag`)
2. `Modernize — Rework — Command discovery and help-navigation`: This family has 1 shipped methods still below compared evidence, so it is the strongest candidate for the next acceptance/harvest deepening sweep. (`weak_evidence_burden`)
