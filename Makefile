# verify {{{
verify\:error: # Check error [syn: verify:err, error, err]
	@cargo check --all --verbose
.PHONY: verify\:error

verify\:err: verify\:error
.PHONY: verify\:err

error: verify\:error
.PHONY: error

err: verify\:error
.PHONY: err

verify\:format: # Show format diff [syn: verify:fmt, format, fmt]
	@cargo fmt --all -- --check
.PHONY: verify\:format

verify\:fmt: verify\:format
.PHONY: verify\:fmt

format: verify\:format
.PHONY: format

fmt: verify\:format
.PHONY: fmt

verify\:lint: # Show suggestions relates to hygiene [syn: lint]
	@cargo clippy --all-targets
.PHONY: verify\:lint

lint: verify\:lint
.PHONY: lint

verify\:all: verify\:error verify\:format verify\:lint # Run all [syn: verify]
.PHONY: verify\:all

verify: verify\:all
.PHONY: verify
# }}}

# test {{{
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
# }}}

# build {{{
build\:debug: # Run packages [syn: build]
	cargo build --workspace
.PHONY: build\:debug

build: build\:debug
.PHONY: build

build\:debug\:cli: # Build only cli package [syn: build:server]
	cargo build --bin nib-server
	.PHONY: build\:debug\:cli

build\:cli: build\:debug\:cli
.PHONY: build\:cli

build\:debug\:lib: # Build only lib package [syn: build:lib]
	cargo build --lib
.PHONY: build\:debug

build\:lib: build\:debug\:lib
.PHONY: build\:lib

build\:debug\:server: # Build only server package [syn: build:server]
	cargo build --bin nib-server
.PHONY: build\:debug\:server

build\:server: build\:debug\:server
.PHONY: build\:server

build\:release: # Build packages with release mode
	cargo build --workspace --release
.PHONY: build\:release

build\:release\:cli: # Build only cli package with release mode
	cargo build --bin cli --release
.PHONY: build\:release\:cli

build\:release\:lib: # Build only lib package with release mode
	cargo build --lib --release
.PHONY: build\:release\:lib

build\:release\:server: # Build only server package with release mode
	cargo build --bin server --release
.PHONY: build\:release\:server
# }}}

# watch {{{
watch\:lib:
	cargo watch --exec 'run --package nib' --delay 0.3
.PHONY: watch\:lib

watch\:cli:
	cargo watch --exec 'run --package nib-cli' --delay 0.3
.PHONY: watch\:cli

watch\:server:
	cargo watch --exec 'run --package nib-server' --delay 0.3
.PHONY: watch\:server
# }}}

# other {{{
clean: # Remove artifacts
	@cargo clean
.PHONY: clean

package\:%: # Create a package of nib, nib-cli or nib-server
	@cargo package --manifest-path src/$(subst package:,,$@)/Cargo.toml
.PHONY: package

install\:%: # Install nib-cli or nib-server into the dir same with cargo
	@cargo install --path src/$(subst package:,,$@) --force
.PHONY: install

help: # Display this message
	@grep --extended-regexp '^[0-9a-z\:\\\%]+: ' $(MAKEFILE_LIST) | \
		grep --extended-regexp ' # ' | \
		sed --expression='s/\([a-z0-9\-\:\ ]*\): \([a-z0-9\-\:\ ]*\) #/\1: #/g' | \
		tr --delete \\\\ | \
		awk 'BEGIN {FS = ": # "}; \
			{printf "\033[38;05;222m%-20s\033[0m %s\n", $$1, $$2}' | \
		sort
.PHONY: help
# }}}

.DEFAULT_GOAL = verify\:all
default: verify\:all
