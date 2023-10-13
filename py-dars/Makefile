.DEFAULT_GOAL := help

PYTHONPATH=
SHELL=/bin/bash
VENV=../.venv

ifeq ($(OS),Windows_NT)
	VENV_BIN=$(VENV)/Scripts
else
	VENV_BIN=$(VENV)/bin
endif

.PHONY: set_python
set_python: ## Configure Python's version con pyenv
	@$(MAKE) -s -C .. $@

.PHONY: .venv
.venv:  ## Set up virtual environment and install requirements
	@$(MAKE) -s -C .. $@

.PHONY: requirements
requirements: .venv  ## Install/refresh all project requirements
	@$(MAKE) -s -C .. $@

.PHONY: clean
clean:  ## Clean up caches and build artifacts
	@rm -rf target/
	@rm -rf docs/build/
	@rm -rf docs/source/reference/api/
	@rm -rf .mypy_cache/
	@rm -rf .pytest_cache/
	@rm -f .coverage
	@rm -f coverage.xml
	@find . -type f -name '*.py[co]' -delete -or -type d -name __pycache__ -delete
	@cargo clean

.PHONY: help
help:  ## Display this help screen
	@echo -e "\033[1mAvailable commands:\033[0m"
	@grep -E '^[a-z.A-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-22s\033[0m %s\n", $$1, $$2}' | sort