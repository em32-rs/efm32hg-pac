all: patch gen

.PHONY: patch clean-rs clean-patch clean-html clean
.PRECIOUS: svd/%.svd .deps/%.d

SHELL := /usr/bin/env bash

CRATES ?= EFM32HG309F64

# All yaml files in devices/ will be used to patch an SVD
YAMLS := $(foreach crate, $(CRATES), \
	       $(wildcard devices/$(crate)*.yaml))

# Each yaml file in devices/ exactly name-matches an SVD file in svd/
PATCHED_SVDS := $(patsubst devices/%.yaml, svd/%.svd.patched, $(YAMLS))

# Turn a devices/device.yaml and svd/device.svd into svd/device.svd.patched
svd/%.svd.patched: devices/%.yaml svd/%.svd .deps/%.d
	python3 scripts/svdpatch.py $<

patch: $(PATCHED_SVDS)

gen:
	cargo gen
	cargo fmt

html/index.html: $(PATCHED_SVDS)
	@mkdir -p html
	python3 scripts/makehtml.py html/ svd/EFM*.svd.patched

html: html/index.html

clean-rs:
	rm -rf $(RUST_DIRS)
	rm -f */src/generic.rs

clean-patch:
	rm -f $(PATCHED_SVDS)

clean-html:
	rm -rf html

clean: clean-rs clean-patch clean-html
	rm -rf .deps

# Generate dependencies for each device YAML
.deps/%.d: devices/%.yaml
	@mkdir -p .deps
	python3 scripts/makedeps.py $< > $@

-include .deps/*
