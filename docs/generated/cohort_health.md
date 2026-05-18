# Cohort Health Gate

This page is generated from the shipped cohort validation report, the governance alignment report, and the current release-candidate readiness document. Review it before reordering future roadmap sweeps.

## Summary

- Shipped methods: `90`
- Compared-evidence methods: `21`
- Methods with harvested legacy provenance recorded: `47`
- Retained backlog still unshipped: `6`
- Largest retained backlog family: `Core Retain — Sequence editing and manipulation` (`6` remaining)
- Weakest evidence family: `Core Retain — Basic sequence IO and conversion` (`15` methods below compared evidence)
- Release-truth document current: `yes`

## Reprioritization Signals

- `dominant_retained_backlog` / `warning`: largest retained backlog remains in 'Core Retain — Sequence editing and manipulation' The governance report shows 6 retained unshipped methods in 'Core Retain — Sequence editing and manipulation'.
- `weak_evidence_burden` / `warning`: 'Core Retain — Basic sequence IO and conversion' carries the largest weak-evidence burden 'Core Retain — Basic sequence IO and conversion' has 15 shipped methods below compared evidence and 3 already compared.

## Ordered Recommendations

1. `Core Retain — Sequence editing and manipulation`: This remains the largest retained backlog family with 6 unshipped methods, so it should stay ahead of smaller backlog sweeps. (`dominant_retained_backlog`)
2. `Core Retain — Basic sequence IO and conversion`: This family has 15 shipped methods still below compared evidence, so it is the strongest candidate for the next acceptance/harvest deepening sweep. (`weak_evidence_burden`)
