PYTHON ?= python3
SPHINXBUILD ?= $(PYTHON) -m sphinx
SPHINXOPTS ?= -n -W --keep-going
DOCS_DIR := docs
DOCS_BUILD_DIR := $(DOCS_DIR)/_build

.PHONY: docs docs-clean

docs:
	$(SPHINXBUILD) $(SPHINXOPTS) -b html $(DOCS_DIR) $(DOCS_BUILD_DIR)/html

docs-clean:
	rm -rf $(DOCS_BUILD_DIR)
