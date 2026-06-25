PYTHON ?= python3
RUSTCARGO ?= cargo
SPHINXBUILD ?= $(PYTHON) -m sphinx
SPHINXOPTS ?= -n -W --keep-going
CONTAINER_RUNTIME ?= docker
RELEASE_VERSION ?= $(shell $(PYTHON) scripts/release_metadata.py workspace-version)
CONTAINER_IMAGE ?= epithema:$(RELEASE_VERSION)
RELEASE_TARGET_OS ?= linux
RELEASE_TARGET_ARCH ?= x86_64
RELEASE_PLATFORM := $(RELEASE_TARGET_OS)-$(RELEASE_TARGET_ARCH)
HOST_OS := $(shell uname -s | tr '[:upper:]' '[:lower:]')
HOST_ARCH := $(shell uname -m | sed -e 's/^amd64$$/x86_64/' -e 's/^aarch64$$/arm64/')
DOCS_DIR := docs
DOCS_BUILD_DIR := $(DOCS_DIR)/_build
DOCS_HTML_DIR := $(DOCS_BUILD_DIR)/html
DOCS_LIVE_PORT ?= 8000
RELEASE_DIST_DIR ?= dist/release/$(RELEASE_VERSION)
RELEASE_BINARY_ARCHIVE := $(RELEASE_DIST_DIR)/epithema-$(RELEASE_VERSION)-$(RELEASE_PLATFORM).tar.gz
RELEASE_DOCS_ARCHIVE := $(RELEASE_DIST_DIR)/epithema-docs-$(RELEASE_VERSION).tar.gz
RELEASE_VALIDATION_ARCHIVE := $(RELEASE_DIST_DIR)/epithema-validation-$(RELEASE_VERSION).tar.gz
RELEASE_MANIFEST := $(RELEASE_DIST_DIR)/epithema-release-manifest.json

.DEFAULT_GOAL := help

.PHONY: help version build fmt lint test docs docs-clean docs-live docs-name-check lint-docs lint-repo check-sister-repo ci clean autodoc-stubs autodoc-refresh generated-index-normalize anchor-validation cohort-report governance-report cohort-health-report comparison-coverage-report full-compared-cohort-report harvest-coverage-report retained-backlog-report release-version-check release-truth-check release-generated-check release-build release-test release-docs release-artifact-platform-check release-artifacts release-container release-check release-clean

help:
	@printf "%s\n" \
		"Epithema project tasks" \
		"" \
		"Documentation:" \
		"  make docs        Build the Sphinx documentation site" \
		"  make autodoc-stubs   Refresh committed autodoc JSON inputs for the exposed tool registry" \
		"  make autodoc-refresh Refresh generated tool Markdown pages from the committed autodoc inputs" \
		"  make anchor-validation Refresh executed-and-compared validation reports for the acceptance anchors" \
		"  make cohort-report   Refresh the shipped cohort validation report JSON and Markdown outputs" \
		"  make governance-report Refresh the governance/backlog versus shipped-registry alignment report" \
		"  make cohort-health-report Refresh the standing roadmap reprioritization gate report" \
		"  make comparison-coverage-report Refresh the family-level compared/executable coverage report" \
		"  make full-compared-cohort-report Refresh the full-compared-cohort release gate report" \
		"  make harvest-coverage-report Refresh the harvest-coverage exceptions report" \
		"  make retained-backlog-report Refresh the retained-backlog closure report" \
		"  make lint-docs   Run strict Sphinx structure and reference checks" \
		"  make docs-name-check Fail if documentation carries retired project names" \
		"  make lint-repo   Validate required repository entry points and docs wiring" \
		"  make check-sister-repo  Inspect ../epithemaR read-only when present" \
		"  make docs-live   Start a live-reloading docs preview (requires sphinx-autobuild)" \
		"  make docs-clean  Remove built documentation output" \
		"" \
		"Rust:" \
		"  make version     Print the checked-in workspace release version" \
		"  make build       Build the Rust workspace" \
		"  make fmt         Check Rust formatting with rustfmt" \
		"  make lint        Run clippy across the workspace" \
		"  make test        Run Rust tests across the workspace" \
		"" \
		"Release:" \
		"  make release-version-check Verify Cargo and Sphinx release metadata alignment" \
		"  make release-truth-check Verify Unreleased and release-note truth-model markers" \
		"  make release-generated-check Refresh governed generated artefacts and require a clean diff" \
		"  make release-build     Build release-mode Rust artefacts" \
		"  make release-test      Run release-gating Rust checks" \
		"  make release-docs      Build release-gating documentation output" \
		"  make release-artifacts Assemble the target-platform/docs/validation release bundle" \
		"  make release-container Build the Linux-first container image" \
		"  make release-check     Run the local release gate" \
		"" \
		"Housekeeping:" \
		"  make ci          Run the current local CI-equivalent checks" \
		"  make clean       Remove generated repository artefacts tracked by this Makefile" \
		"  make release-clean Remove local release bundle output under dist/release/" \

version:
	@printf "%s\n" "$(RELEASE_VERSION)"

# Documentation
build:
	$(RUSTCARGO) build --workspace

fmt:
	$(RUSTCARGO) fmt --check

lint:
	$(RUSTCARGO) clippy --workspace --all-targets --all-features

test:
	$(RUSTCARGO) test --workspace --all-features

release-version-check:
	$(PYTHON) scripts/release_metadata.py check

release-truth-check:
	$(PYTHON) scripts/release_metadata.py truth-check

release-generated-check:
	rm -rf docs/generated/tools docs/generated/index.md
	@for doc in $$(find docs/autodoc/tools -name '*.json' | sort); do \
		printf "%s\n" "Refreshing $$doc"; \
		$(RUSTCARGO) run -p epithema-cli -- autodoc "$$doc" --emit-docs --emit-validation-stub >/dev/null; \
	done
	$(MAKE) generated-index-normalize PYTHON=$(PYTHON)
	$(MAKE) cohort-report
	$(MAKE) governance-report
	$(MAKE) cohort-health-report
	$(MAKE) comparison-coverage-report
	$(MAKE) full-compared-cohort-report
	$(MAKE) harvest-coverage-report
	$(MAKE) retained-backlog-report
	git diff --exit-code -- docs/generated

release-build:
	$(RUSTCARGO) build --workspace --release

release-test: fmt lint test

release-docs:
	$(MAKE) release-version-check PYTHON=$(PYTHON)
	$(MAKE) docs-clean
	$(MAKE) docs PYTHON=$(PYTHON)
	$(MAKE) docs-name-check

release-artifact-platform-check:
	@if [ "$(HOST_OS)" != "$(RELEASE_TARGET_OS)" ] || [ "$(HOST_ARCH)" != "$(RELEASE_TARGET_ARCH)" ]; then \
		printf "%s\n" "release artifact platform mismatch:"; \
		printf "%s\n" "  host: $(HOST_OS)-$(HOST_ARCH)"; \
		printf "%s\n" "  target: $(RELEASE_PLATFORM)"; \
		printf "%s\n" "Run this target on the requested release platform or override RELEASE_TARGET_OS/RELEASE_TARGET_ARCH intentionally."; \
		exit 1; \
	fi

release-artifacts: release-version-check release-artifact-platform-check release-build release-docs cohort-report
	mkdir -p $(RELEASE_DIST_DIR)
	tar -C target/release -czf $(RELEASE_BINARY_ARCHIVE) epithema
	sha256sum $(RELEASE_BINARY_ARCHIVE) > $(RELEASE_BINARY_ARCHIVE).sha256
	tar -C $(DOCS_BUILD_DIR) -czf $(RELEASE_DOCS_ARCHIVE) html
	tar -czf $(RELEASE_VALIDATION_ARCHIVE) \
		docs/generated/cohort_validation.md \
		docs/generated/validation
	$(PYTHON) scripts/release_metadata.py manifest \
		--output $(RELEASE_MANIFEST) \
		--binary-platform $(RELEASE_PLATFORM) \
		--container-image $(CONTAINER_IMAGE)

release-container:
	$(CONTAINER_RUNTIME) build \
		--build-arg EPITHEMA_VERSION=$(RELEASE_VERSION) \
		-t $(CONTAINER_IMAGE) \
		.

release-check: lint-repo check-sister-repo release-version-check release-truth-check release-generated-check release-test release-docs release-build

docs:
	$(SPHINXBUILD) $(SPHINXOPTS) -b html $(DOCS_DIR) $(DOCS_HTML_DIR)

docs-name-check:
	@pattern='EMBOSS-RS|emboss-rs|emboss_rs|emboss-r\b'; \
	if rg -n -i --hidden --glob '!docs/_build/**' --glob '!target/**' --glob '!.git/**' "$$pattern" .github docs README.md; then \
		printf "%s\n" "retired project name found in checked documentation or workflow source" >&2; \
		exit 1; \
	fi; \
	if [ -d "$(DOCS_HTML_DIR)" ] && rg -n -i "$$pattern" "$(DOCS_HTML_DIR)"; then \
		printf "%s\n" "retired project name found in built Sphinx HTML" >&2; \
		exit 1; \
	fi

autodoc-stubs:
	$(RUSTCARGO) run -p epithema-docgen --example write_registry_autodoc_stubs -- docs/autodoc/tools

autodoc-refresh: autodoc-stubs
	rm -rf docs/generated/tools docs/generated/index.md
	@for doc in $$(find docs/autodoc/tools -name '*.json' | sort); do \
		printf "%s\n" "Refreshing $$doc"; \
		$(RUSTCARGO) run -p epithema-cli -- autodoc "$$doc" --emit-docs >/dev/null; \
	done
	$(MAKE) generated-index-normalize

generated-index-normalize:
	$(PYTHON) scripts/normalize_generated_index.py

cohort-report:
	$(MAKE) anchor-validation
	$(RUSTCARGO) run -p epithema-testkit --example write_shipped_cohort_validation_report -- \
		--json docs/generated/validation/shipped_cohort.validation.json \
		--markdown docs/generated/cohort_validation.md

governance-report:
	$(RUSTCARGO) run -p epithema-testkit --example write_governance_alignment_report -- \
		--json docs/generated/validation/governance_alignment.json \
		--markdown docs/generated/governance_alignment.md

cohort-health-report:
	$(RUSTCARGO) run -p epithema-testkit --example write_cohort_health_report -- \
		--json docs/generated/validation/cohort_health.json \
		--markdown docs/generated/cohort_health.md

comparison-coverage-report:
	$(RUSTCARGO) run -p epithema-testkit --example write_comparison_coverage_report -- \
		--json docs/generated/validation/comparison_coverage.json \
		--markdown docs/generated/comparison_coverage.md

full-compared-cohort-report:
	$(RUSTCARGO) run -p epithema-testkit --example write_full_compared_cohort_report -- \
		--json docs/generated/validation/full_compared_cohort.json \
		--markdown docs/generated/full_compared_cohort.md

harvest-coverage-report:
	$(RUSTCARGO) run -p epithema-testkit --example write_harvest_coverage_report -- \
		--json docs/generated/validation/harvest_coverage.json \
		--markdown docs/generated/harvest_coverage.md

retained-backlog-report:
	$(RUSTCARGO) run -p epithema-testkit --example write_retained_backlog_report -- \
		--json docs/generated/validation/retained_backlog_closure.json \
		--markdown docs/generated/retained_backlog_closure.md

anchor-validation:
	$(RUSTCARGO) run -p epithema-testkit --example write_acceptance_anchor_reports -- \
		--output-dir docs/generated/validation

lint-docs:
	$(SPHINXBUILD) $(SPHINXOPTS) -b dummy $(DOCS_DIR) $(DOCS_BUILD_DIR)/lint

lint-repo:
	test -f README.md
	test -f Makefile
	test -f Cargo.toml
	test -f docs/index.md
	test -f docs/README.md
	test -f docs/autodoc/README.md
	test -f docs/generated/index.md
	test -f docs/governance/index.md
	test -f docs/governance/epithema_governance_manual.md
	test -f .github/workflows/docs-pages.yml
	test -f crates/epithema-cli/Cargo.toml
	grep -n "^governance/index$$" docs/index.md
	grep -n "epithema_governance_manual" docs/governance/index.md docs/README.md README.md
	grep -n "epithemaR" README.md docs/governance/epithema_governance_manual.md docs/governance/appendices/foundational_architecture_brief.md

check-sister-repo:
	@if [ -d ../epithemaR ]; then \
		printf "%s\n" "Found sibling repository: ../epithemaR"; \
		test -f ../epithemaR/README.md; \
		grep -n "^# epithemaR$$" ../epithemaR/README.md; \
		grep -n "plots\|methods available in R\|R" ../epithemaR/README.md; \
	else \
		printf "%s\n" "../epithemaR is not present in this environment; skipping read-only compatibility awareness check."; \
	fi

docs-live:
	@$(PYTHON) -m sphinx_autobuild --version >/dev/null 2>&1 || { \
		printf "%s\n" \
			"sphinx-autobuild is not installed in the selected Python environment." \
			"Install it with: $(PYTHON) -m pip install sphinx-autobuild"; \
		exit 1; \
	}
	$(PYTHON) -m sphinx_autobuild \
		--port $(DOCS_LIVE_PORT) \
		$(DOCS_DIR) \
		$(DOCS_HTML_DIR)

docs-clean:
	rm -rf $(DOCS_BUILD_DIR)

# Housekeeping
clean: docs-clean

release-clean:
	rm -rf dist/release

ci: lint-repo check-sister-repo lint-docs docs docs-name-check fmt lint test

# Future extension points:
# - autodoc
# - validate
# - release
# - container
