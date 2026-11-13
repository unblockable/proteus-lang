SHELL := bash
.ONESHELL:
.SHELLFLAGS := -eu -o pipefail -c
.DELETE_ON_ERROR:
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules
.DEFAULT_GOAL := build

CARGO ?= cargo

build:
	$(CARGO) build
.PHONY: build

check:
	$(CARGO) check --all-targets
.PHONY: check

lint:
	$(CARGO) clippy --all-targets -- -D warnings
.PHONY: lint

format:
	$(CARGO) fmt --all
.PHONY: format

test:
	$(CARGO) test
.PHONY: test

# Requires the cargo-watch command-line tool.
watch-test:
	$(CARGO) watch -x test
.PHONY: watch-test
