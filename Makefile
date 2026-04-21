PYTHON ?= python3
RUSTCARGO ?= cargo
SPHINXBUILD ?= $(PYTHON) -m sphinx
SPHINXOPTS ?= -n -W --keep-going
CONTAINER_RUNTIME ?= docker
RELEASE_VERSION ?= $(shell $(PYTHON) scripts/release_metadata.py workspace-version)
CONTAINER_IMAGE ?= emboss-rs:$(RELEASE_VERSION)
DOCS_DIR := docs
DOCS_BUILD_DIR := $(DOCS_DIR)/_build
DOCS_HTML_DIR := $(DOCS_BUILD_DIR)/html
DOCS_LIVE_PORT ?= 8000
RELEASE_DIST_DIR ?= dist/release/$(RELEASE_VERSION)
RELEASE_BINARY_ARCHIVE := $(RELEASE_DIST_DIR)/emboss-rs-$(RELEASE_VERSION)-linux-x86_64.tar.gz
RELEASE_DOCS_ARCHIVE := $(RELEASE_DIST_DIR)/emboss-rs-docs-$(RELEASE_VERSION).tar.gz
RELEASE_VALIDATION_ARCHIVE := $(RELEASE_DIST_DIR)/emboss-rs-validation-$(RELEASE_VERSION).tar.gz
RELEASE_MANIFEST := $(RELEASE_DIST_DIR)/emboss-rs-release-manifest.json

.DEFAULT_GOAL := help

.PHONY: help version build fmt lint test docs docs-clean docs-live lint-docs lint-repo check-sister-repo ci clean autodoc-stubs autodoc-refresh generated-index-normalize cohort-report release-version-check release-generated-check release-build release-test release-docs release-artifacts release-container release-check release-clean

help:
	@printf "%s\n" \
		"EMBOSS-RS project tasks" \
		"" \
		"Documentation:" \
		"  make docs        Build the Sphinx documentation site" \
		"  make autodoc-stubs   Refresh committed autodoc JSON inputs for the exposed tool registry" \
		"  make autodoc-refresh Refresh generated tool Markdown pages from the committed autodoc inputs" \
		"  make cohort-report   Refresh the shipped cohort validation report JSON and Markdown outputs" \
		"  make lint-docs   Run strict Sphinx structure and reference checks" \
		"  make lint-repo   Validate required repository entry points and docs wiring" \
		"  make check-sister-repo  Inspect ../emboss-r read-only when present" \
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
		"  make release-generated-check Refresh governed generated artefacts and require a clean diff" \
		"  make release-build     Build release-mode Rust artefacts" \
		"  make release-test      Run release-gating Rust checks" \
		"  make release-docs      Build release-gating documentation output" \
		"  make release-artifacts Assemble the local Linux/docs/validation release bundle" \
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

release-generated-check:
	rm -rf docs/generated/tools docs/generated/index.md
	@for doc in $$(find docs/autodoc/tools -name '*.json' | sort); do \
		printf "%s\n" "Refreshing $$doc"; \
		$(RUSTCARGO) run -p emboss-cli -- autodoc "$$doc" --emit-docs --emit-validation-stub >/dev/null; \
	done
	$(MAKE) generated-index-normalize PYTHON=$(PYTHON)
	$(MAKE) cohort-report
	git diff --exit-code -- docs/generated

release-build:
	$(RUSTCARGO) build --workspace --release

release-test: fmt lint test

release-docs:
	$(MAKE) release-version-check PYTHON=$(PYTHON)
	$(MAKE) docs-clean
	$(MAKE) docs PYTHON=$(PYTHON)

release-artifacts: release-version-check release-build release-docs cohort-report
	mkdir -p $(RELEASE_DIST_DIR)
	tar -C target/release -czf $(RELEASE_BINARY_ARCHIVE) emboss-rs
	sha256sum $(RELEASE_BINARY_ARCHIVE) > $(RELEASE_BINARY_ARCHIVE).sha256
	tar -C $(DOCS_BUILD_DIR) -czf $(RELEASE_DOCS_ARCHIVE) html
	tar -czf $(RELEASE_VALIDATION_ARCHIVE) \
		docs/generated/cohort_validation.md \
		docs/generated/validation
	$(PYTHON) scripts/release_metadata.py manifest \
		--output $(RELEASE_MANIFEST) \
		--container-image $(CONTAINER_IMAGE)

release-container:
	$(CONTAINER_RUNTIME) build \
		--build-arg EMBOSS_RS_VERSION=$(RELEASE_VERSION) \
		-t $(CONTAINER_IMAGE) \
		.

release-check: lint-repo check-sister-repo release-version-check release-generated-check release-test release-docs release-build

docs:
	$(SPHINXBUILD) $(SPHINXOPTS) -b html $(DOCS_DIR) $(DOCS_HTML_DIR)

autodoc-stubs:
	$(RUSTCARGO) run -p emboss-docgen --example write_registry_autodoc_stubs -- docs/autodoc/tools

autodoc-refresh: autodoc-stubs
	rm -rf docs/generated/tools docs/generated/index.md
	@for doc in $$(find docs/autodoc/tools -name '*.json' | sort); do \
		printf "%s\n" "Refreshing $$doc"; \
		$(RUSTCARGO) run -p emboss-cli -- autodoc "$$doc" --emit-docs >/dev/null; \
	done
	$(MAKE) generated-index-normalize

generated-index-normalize:
	$(PYTHON) scripts/normalize_generated_index.py

cohort-report:
	$(RUSTCARGO) run -p emboss-testkit --example write_shipped_cohort_validation_report -- \
		--json docs/generated/validation/shipped_cohort.validation.json \
		--markdown docs/generated/cohort_validation.md

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
	test -f docs/governance/emboss_rs_governance_manual.md
	test -f .github/workflows/docs-pages.yml
	test -f crates/emboss-cli/Cargo.toml
	grep -n "^governance/index$$" docs/index.md
	grep -n "emboss_rs_governance_manual" docs/governance/index.md docs/README.md README.md
	grep -n "emboss-r" README.md docs/governance/emboss_rs_governance_manual.md docs/governance/appendices/foundational_architecture_brief.md

check-sister-repo:
	@if [ -d ../emboss-r ]; then \
		printf "%s\n" "Found sibling repository: ../emboss-r"; \
		test -f ../emboss-r/README.md; \
		grep -n "^# emboss-r$$" ../emboss-r/README.md; \
		grep -n "plots\|methods available in R\|R" ../emboss-r/README.md; \
	else \
		printf "%s\n" "../emboss-r is not present in this environment; skipping read-only compatibility awareness check."; \
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

ci: lint-repo check-sister-repo lint-docs docs fmt lint test

# Future extension points:
# - autodoc
# - validate
# - release
# - container
