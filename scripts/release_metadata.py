#!/usr/bin/env python3
"""Release metadata helpers for emboss-rs."""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
CARGO_TOML = REPO_ROOT / "Cargo.toml"
DOCS_CONF = REPO_ROOT / "docs" / "conf.py"
CHANGELOG = REPO_ROOT / "CHANGELOG.md"
RELEASE_NOTES = REPO_ROOT / "docs" / "release" / "v1_0_0_release_notes.md"
RC_READINESS = REPO_ROOT / "docs" / "release" / "v1_0_0_rc_readiness.md"
COHORT_VALIDATION = REPO_ROOT / "docs" / "generated" / "validation" / "shipped_cohort.validation.json"
GOVERNANCE_ALIGNMENT = REPO_ROOT / "docs" / "generated" / "validation" / "governance_alignment.json"
COHORT_HEALTH = REPO_ROOT / "docs" / "generated" / "validation" / "cohort_health.json"
COMPARISON_COVERAGE = REPO_ROOT / "docs" / "generated" / "validation" / "comparison_coverage.json"
RETAINED_BACKLOG_CLOSURE = REPO_ROOT / "docs" / "generated" / "validation" / "retained_backlog_closure.json"

REQUIRED_UNRELEASED_MARKERS = [
    "## [Unreleased]",
    "New shipped tools after `1.0.0` must not bypass:",
    "- governance mapping",
    "- autodoc presence",
    "- validation-stub generation",
    "- cohort-report inclusion",
    "- at least one compared anchor per shipped family",
    "- comparison-coverage reporting",
    "- retained-backlog-closure reporting",
    "- drift-free release-facing counts and report links",
    "- honest release-note wording",
]

REQUIRED_RELEASE_NOTES_MARKERS = [
    "These notes remain a draft release document",
    "[Cohort Validation Report](../generated/cohort_validation.md)",
    "[Governance Alignment Report](../generated/governance_alignment.md)",
    "[Cohort Health Gate](../generated/cohort_health.md)",
    "[Comparison Coverage Report](../generated/comparison_coverage.md)",
    "[Retained Backlog Closure Report](../generated/retained_backlog_closure.md)",
]

REQUIRED_RC_READINESS_MARKERS = [
    "- `make comparison-coverage-report`",
    "- `make retained-backlog-report`",
    "- `docs/generated/validation/comparison_coverage.json`",
    "- `docs/generated/comparison_coverage.md`",
    "- `docs/generated/validation/retained_backlog_closure.json`",
    "- `docs/generated/retained_backlog_closure.md`",
]


def load_workspace_version() -> str:
    match = re.search(
        r'^\[workspace\.package\]\n(?:(?!^\[).*\n)*?version\s*=\s*"(?P<version>[^"]+)"\s*$',
        CARGO_TOML.read_text(),
        flags=re.MULTILINE,
    )
    if match is None:
        raise SystemExit(f"could not find workspace.package.version in {CARGO_TOML}")
    return match.group("version")


def load_docs_release() -> str:
    match = re.search(
        r'^release\s*=\s*"(?P<version>[^"]+)"\s*$',
        DOCS_CONF.read_text(),
        flags=re.MULTILINE,
    )
    if match is None:
        raise SystemExit(f"could not find Sphinx release in {DOCS_CONF}")
    return match.group("version")


def check_version_alignment() -> int:
    workspace_version = load_workspace_version()
    docs_release = load_docs_release()
    if workspace_version != docs_release:
        raise SystemExit(
            "release metadata mismatch: "
            f"workspace.package.version={workspace_version!r}, "
            f"docs.conf release={docs_release!r}"
        )
    return 0


def load_json(path: Path) -> dict:
    try:
        return json.loads(path.read_text())
    except FileNotFoundError as exc:
        raise SystemExit(f"release truth check failed:\n- missing required report {path}") from exc


def extract_count(
    text: str,
    pattern: str,
    source: Path,
    label: str,
    missing: list[str],
) -> int | None:
    match = re.search(pattern, text, flags=re.MULTILINE)
    if match is None:
        missing.append(f"{source}: missing {label}")
        return None
    return int(match.group("count"))


def check_release_truth_surface() -> int:
    changelog_text = CHANGELOG.read_text()
    release_notes_text = RELEASE_NOTES.read_text()
    rc_readiness_text = RC_READINESS.read_text()
    cohort_report = load_json(COHORT_VALIDATION)
    governance_report = load_json(GOVERNANCE_ALIGNMENT)
    cohort_health_report = load_json(COHORT_HEALTH)
    comparison_coverage_report = load_json(COMPARISON_COVERAGE)
    retained_backlog_report = load_json(RETAINED_BACKLOG_CLOSURE)
    cohort_summary = cohort_report["summary"]
    governance_summary = governance_report["summary"]
    health_summary = cohort_health_report["summary"]
    comparison_summary = comparison_coverage_report["summary"]
    retained_backlog_summary = retained_backlog_report["summary"]

    missing = []
    for marker in REQUIRED_UNRELEASED_MARKERS:
        if marker not in changelog_text:
            missing.append(f"{CHANGELOG}: missing marker {marker!r}")

    for marker in REQUIRED_RELEASE_NOTES_MARKERS:
        if marker not in release_notes_text:
            missing.append(f"{RELEASE_NOTES}: missing marker {marker!r}")

    for marker in REQUIRED_RC_READINESS_MARKERS:
        if marker not in rc_readiness_text:
            missing.append(f"{RC_READINESS}: missing marker {marker!r}")

    release_notes_counts = {
        "shipped methods": extract_count(
            release_notes_text,
            r"cohort\s+of\s+`(?P<count>\d+)`\s+methods",
            RELEASE_NOTES,
            "shipped-method count marker",
            missing,
        ),
        "compared evidence": extract_count(
            release_notes_text,
            r"- `(?P<count>\d+)` shipped methods carry compared evidence",
            RELEASE_NOTES,
            "compared-evidence count marker",
            missing,
        ),
        "executable evidence": extract_count(
            release_notes_text,
            r"- `(?P<count>\d+)` shipped methods carry executable evidence",
            RELEASE_NOTES,
            "executable-evidence count marker",
            missing,
        ),
        "harvested legacy provenance": extract_count(
            release_notes_text,
            r"- `(?P<count>\d+)` shipped methods record harvested legacy provenance",
            RELEASE_NOTES,
            "harvested-legacy count marker",
            missing,
        ),
        "retained backlog": extract_count(
            release_notes_text,
            r"- `(?P<count>\d+)` retained governance methods remain unshipped",
            RELEASE_NOTES,
            "retained-backlog count marker",
            missing,
        ),
    }
    rc_readiness_counts = {
        "shipped methods": extract_count(
            rc_readiness_text,
            r"- Shipped methods audited: `(?P<count>\d+)`",
            RC_READINESS,
            "shipped-method readiness marker",
            missing,
        ),
        "compared evidence": extract_count(
            rc_readiness_text,
            r"- Compared-evidence methods: `(?P<count>\d+)`",
            RC_READINESS,
            "compared-evidence readiness marker",
            missing,
        ),
        "executable evidence": extract_count(
            rc_readiness_text,
            r"- Executable-evidence methods: `(?P<count>\d+)`",
            RC_READINESS,
            "executable-evidence readiness marker",
            missing,
        ),
        "harvested legacy provenance": extract_count(
            rc_readiness_text,
            r"- Methods with harvested legacy provenance recorded: `(?P<count>\d+)`",
            RC_READINESS,
            "harvested-legacy readiness marker",
            missing,
        ),
        "retained backlog": extract_count(
            rc_readiness_text,
            r"- Retained backlog still unshipped: `(?P<count>\d+)`",
            RC_READINESS,
            "retained-backlog readiness marker",
            missing,
        ),
    }

    expected_counts = {
        "shipped methods": cohort_summary["total_method_count"],
        "compared evidence": cohort_summary["compared_evidence_count"],
        "executable evidence": cohort_summary["executable_evidence_count"],
        "harvested legacy provenance": cohort_summary["harvested_legacy_presence_count"],
        "retained backlog": governance_summary["retained_backlog_count"],
    }
    for label, expected in expected_counts.items():
        observed = release_notes_counts.get(label)
        if observed is not None and observed != expected:
            missing.append(
                f"{RELEASE_NOTES}: {label} marker is {observed}, expected {expected}"
            )
        observed = rc_readiness_counts.get(label)
        if observed is not None and observed != expected:
            missing.append(
                f"{RC_READINESS}: {label} marker is {observed}, expected {expected}"
            )

    consistency_checks = [
        (
            governance_summary["shipped_tool_count"],
            cohort_summary["total_method_count"],
            "governance shipped-tool count vs cohort total-method count",
        ),
        (
            governance_summary["shipped_compared_count"],
            cohort_summary["compared_evidence_count"],
            "governance shipped-compared count vs cohort compared count",
        ),
        (
            governance_summary["shipped_harvested_legacy_presence_count"],
            cohort_summary["harvested_legacy_presence_count"],
            "governance harvested-legacy count vs cohort harvested-legacy count",
        ),
        (
            governance_summary["retained_backlog_count"],
            health_summary["retained_backlog_count"],
            "governance retained backlog vs cohort-health retained backlog",
        ),
        (
            governance_summary["retained_backlog_count"],
            retained_backlog_summary["retained_backlog_count"],
            "governance retained backlog vs retained-backlog-closure retained backlog",
        ),
        (
            health_summary["total_method_count"],
            cohort_summary["total_method_count"],
            "cohort-health total-method count vs cohort total-method count",
        ),
        (
            health_summary["compared_evidence_count"],
            cohort_summary["compared_evidence_count"],
            "cohort-health compared count vs cohort compared count",
        ),
        (
            health_summary["harvested_legacy_presence_count"],
            cohort_summary["harvested_legacy_presence_count"],
            "cohort-health harvested-legacy count vs cohort harvested-legacy count",
        ),
        (
            comparison_summary["total_method_count"],
            cohort_summary["total_method_count"],
            "comparison-coverage total-method count vs cohort total-method count",
        ),
        (
            comparison_summary["compared_count"],
            cohort_summary["compared_evidence_count"],
            "comparison-coverage compared count vs cohort compared count",
        ),
        (
            comparison_summary["executable_only_count"],
            cohort_summary["executable_evidence_count"],
            "comparison-coverage executable-only count vs cohort executable count",
        ),
        (
            retained_backlog_summary["retained_tool_count"],
            governance_summary["retained_tool_count"],
            "retained-backlog retained-tool count vs governance retained-tool count",
        ),
        (
            retained_backlog_summary["retained_shipped_count"],
            governance_summary["shipped_retain_count"],
            "retained-backlog retained-shipped count vs governance shipped-retain count",
        ),
    ]
    for left, right, label in consistency_checks:
        if left != right:
            missing.append(f"cross-report drift: {label} ({left} != {right})")

    if comparison_summary["harvested_but_not_compared_count"] > cohort_summary["harvested_legacy_presence_count"]:
        missing.append(
            "cross-report drift: comparison-coverage harvested-but-not-compared "
            "count exceeds cohort harvested-legacy count"
        )

    if not health_summary["release_truth_current"]:
        missing.append(
            f"{COHORT_HEALTH}: release_truth_current must remain true before release gating passes"
        )

    if governance_summary["retained_backlog_count"] == 0:
        if not retained_backlog_summary["retained_backlog_closed"]:
            missing.append(
                f"{RETAINED_BACKLOG_CLOSURE}: retained_backlog_closed must be true when retained backlog is zero"
            )
        for row in comparison_coverage_report["families"]:
            if row["shipped_method_count"] > 0 and row["compared_count"] < 1:
                missing.append(
                    "comparison coverage gate failed: "
                    f"{row['family']!r} has shipped methods but no compared anchor"
                )

    if missing:
        raise SystemExit(
            "release truth check failed:\n- " + "\n- ".join(missing)
        )
    return 0


def write_manifest(output: Path, container_image: str | None) -> int:
    version = load_workspace_version()
    manifest = {
        "schema_version": 1,
        "project": "emboss-rs",
        "version": version,
        "binary_name": "emboss-rs",
        "artifacts": {
            "linux_binary_archive": f"emboss-rs-{version}-linux-x86_64.tar.gz",
            "linux_binary_checksum": f"emboss-rs-{version}-linux-x86_64.tar.gz.sha256",
            "docs_archive": f"emboss-rs-docs-{version}.tar.gz",
            "validation_archive": f"emboss-rs-validation-{version}.tar.gz",
        },
        "validation_report": {
            "json": "docs/generated/validation/shipped_cohort.validation.json",
            "markdown": "docs/generated/cohort_validation.md",
        },
        "docs": {
            "sphinx_release": load_docs_release(),
            "pages_path": "docs/_build/html",
        },
    }
    if container_image:
        manifest["container_image"] = container_image
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(manifest, indent=2) + "\n")
    return 0


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(dest="command", required=True)

    subparsers.add_parser("workspace-version", help="Print the workspace release version")
    subparsers.add_parser("check", help="Verify release metadata alignment")
    subparsers.add_parser(
        "truth-check",
        help="Verify Unreleased and release-note wording preserves the release-truth model",
    )

    manifest = subparsers.add_parser(
        "manifest",
        help="Write a release manifest describing the reproducible local artefact set",
    )
    manifest.add_argument("--output", required=True, type=Path)
    manifest.add_argument("--container-image", default=None)

    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)

    if args.command == "workspace-version":
        print(load_workspace_version())
        return 0
    if args.command == "check":
        return check_version_alignment()
    if args.command == "truth-check":
        return check_release_truth_surface()
    if args.command == "manifest":
        return write_manifest(args.output, args.container_image)

    parser.error(f"unsupported command: {args.command}")
    return 2


if __name__ == "__main__":
    sys.exit(main())
