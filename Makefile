# vet
vet\:error: # Check error [synonym: vet:err, error, err]
	@cargo check --all --verbose
.PHONY: vet\:error

vet\:err: vet\:error
.PHONY: vet\:err

error: vet\:error
.PHONY: error

err: vet\:error
.PHONY: err

vet\:format: # Show format diff [synonym: vet:fmt, format, fmt]
	@cargo fmt --all -- --check
.PHONY: vet\:format

vet\:fmt: vet\:format
.PHONY: vet\:fmt

format: vet\:format
.PHONY: format

fmt: vet\:format
.PHONY: fmt

vet\:lint: # Show suggestions relates to hygiene [synonym: lint]
	@cargo clippy --all-targets
.PHONY: vet\:lint

lint: vet\:lint
.PHONY: lint

vet\:all: err fmt lint # Run all vet targets
.PHONY: vet\:all

vet: vet\:check # Alias for vet:check
.PHONY: vet

# test
test\:unit:
	@cargo test --lib --bins
.PHONY: test\:unit

test\:integration:
	@cargo test --test integration
.PHONY: test\:integration

test\:all:
	@cargo test --lib --bins --test integration
.PHONY: test\:all

test: test\:all
.PHONY: test

# build
build\:debug: # Run packages [synonym: build]
	cargo build --workspace
.PHONY: build\:debug

build: build\:debug
.PHONY: build

build\:debug\:cli: # Build only cli package [synonym: build:cli]
	cargo build --bin nib
.PHONY: build\:debug\:cli

build\:cli: build\:debug\:cli
.PHONY: build\:cli

build\:debug\:lib: # Build only lib package [synonym: build:lib]
	cargo build --lib
.PHONY: build\:debug\:lib

build\:lib: build\:debug\:lib
.PHONY: build\:lib

build\:debug\:server: # Build only server package [synonym: build:server]
	cargo build --bin nib-server
.PHONY: build\:debug\:server

build\:server: build\:debug\:server
.PHONY: build\:server

build\:release: # Build packages with release mode
	cargo build --workspace --release
.PHONY: build\:release

build\:release\:cli: # Build only cli package with release mode
	cargo build --package nib-cli --bin nib --release
.PHONY: build\:release\:cli

build\:release\:lib: # Build only lib package with release mode
	cargo build --package nib --lib --release
.PHONY: build\:release\:lib

build\:release\:server: # Build only server package with release mode
	cargo build --package nib-server --bin nib-server --release
.PHONY: build\:release\:server

# utility
watch\:lib:
	cargo watch --exec 'run --package nib' --delay 0.3
.PHONY: watch\:lib

watch\:cli:
	cargo watch --exec 'run --package nib-cli' --delay 0.3
.PHONY: watch\:cli

watch\:server:
	cargo watch --exec 'run --package nib-server' --delay 0.3
.PHONY: watch\:server

clean: # Remove artifacts
	@cargo clean
.PHONY: clean

package\:%: # Create a package of nib, nib-cli or nib-server
	@cargo package --manifest-path src/$(subst package:,,$@)/Cargo.toml
.PHONY: package

install\:%: # Install nib-cli or nib-server into the dir same with cargo
	@cargo install --path src/$(subst install:,,$@) --force
.PHONY: install

help: # Display this message
	@set -uo pipefail; \
	grep --extended-regexp '^[0-9a-z\:\\\%]+: ' \
		$(firstword $(MAKEFILE_LIST)) | \
		grep --extended-regexp ' # ' | \
		sed --expression='s/\([a-z0-9\-\:\ ]*\): \([a-z0-9\-\:\ ]*\) #/\1: #/g' | \
		tr --delete \\\\ | \
		awk 'BEGIN {FS = ": # "}; \
			{printf "\033[38;05;222m%-21s\033[0m %s\n", $$1, $$2}' | \
		sort
.PHONY: help

.DEFAULT_GOAL = vet\:all
default: vet\:all
