# verify {{{
verify\:check:  ## Check syntax [alias: check]
	@cargo check --all --verbose
.PHONY: verify\:check

check: | verify\:check
.PHONY: check

verify\:format:  ## Check format without changes [alias: verify:fmt, format, fmt]
	@cargo fmt --all -- --check
.PHONY: verify\:format

verify\:fmt: | verify\:format
.PHONY: verify\:fmt

format: | verify\:format
.PHONY: format

fmt: | verify\:format
.PHONY: fmt

verify\:lint:  ## Check coding-style [alias: lint]
	@cargo clippy --all-targets
.PHONY: verify\:lint

lint: | verify\:lint
.PHONY: lint

verify\:all: | verify\:check verify\:format verify\:lint  ## Run all verify targets [alias: verify]
.PHONY: verify\:all

verify: | verify\:all
.PHONY: verify
# }}}

# build {{{
build\:debug:  ## Run debug build [alias: build]
	cargo build
.PHONY: build\:debug

build\:debug\:server:  ## Build a development server [alias: build:server]
	cargo build --bin beta-server
.PHONY: build\:release

build\:server: | build\:debug\:server
.PHONY: build\:server

build: | build\:debug
.PHONY: build

build\:release:  ## Create release build
	cargo build --release
.PHONY: build\:release

build\:release\:server:  ## Build a development server with release mode
	cargo build --bin server --release
	.PHONY: build\:release\:server
# }}}

# server {{{
watch\:server:
	cargo build --exec 'run --bin server' --delay 0.3
.PHONY: watch\:server
# }}}

# other {{{
clean:  ## Tidy up
	@cargo clean
.PHONY: clean

package:  ## Create package
	@cargo package
.PHONY: package

install:  ## Install built binary into the directory same with cargo
	@cargo install --path . --force
.PHONY: install

help:  ## Display this message
	@grep --extended-regexp '^[0-9a-z\:\\\%]+: ' $(MAKEFILE_LIST) | \
		grep --extended-regexp '  ## ' | \
		sed --expression='s/\(\s|\(\s[-_0-9a-z\:\\]*\)*\)  /  /g' | \
		tr --delete \\\\ | \
		awk 'BEGIN {FS = ":  ## "}; \
			{printf "\033[38;05;222m%-13s\033[0m %s\n", $$1, $$2}' | \
		sort
.PHONY: help
# }}}

.DEFAULT_GOAL = verify\:all
default: verify\:all
