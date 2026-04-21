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
    if args.command == "manifest":
        return write_manifest(args.output, args.container_image)

    parser.error(f"unsupported command: {args.command}")
    return 2


if __name__ == "__main__":
    sys.exit(main())
