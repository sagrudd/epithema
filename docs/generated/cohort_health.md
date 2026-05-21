# Cohort Health Gate

This page is generated from the shipped cohort validation report, the governance alignment report, and the current release-candidate readiness document. Review it before reordering future roadmap sweeps.

## Summary

- Shipped methods: `96`
- Compared-evidence methods: `61`
- Methods with harvested legacy provenance recorded: `90`
- Retained backlog still unshipped: `0`
- Largest retained backlog family: `none` (`0` remaining)
- Weakest evidence family: `Core Retain — Core sequence statistics and composition` (`10` methods below compared evidence)
- Release-truth document current: `no`

## Reprioritization Signals

- `weak_evidence_burden` / `warning`: 'Core Retain — Core sequence statistics and composition' carries the largest weak-evidence burden 'Core Retain — Core sequence statistics and composition' has 10 shipped methods below compared evidence and 6 already compared.
- `release_truth_lag` / `warning`: release-truth documentation is behind the current generated state The RC readiness document is missing current markers for: - Compared-evidence methods: `61`, - Executable-evidence methods: `35`, - Methods with harvested legacy provenance recorded: `90`.

## Ordered Recommendations

1. `release readiness truth`: Refresh the RC readiness material before adding more shipped scope so release-facing documentation does not lag the generated cohort state. (`release_truth_lag`)
2. `Core Retain — Core sequence statistics and composition`: This family has 10 shipped methods still below compared evidence, so it is the strongest candidate for the next acceptance/harvest deepening sweep. (`weak_evidence_burden`)
