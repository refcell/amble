# Heavily inspired by Reth
# https://github.com/paradigmxyz/reth/blob/main/Makefile
.DEFAULT_GOAL := help

GIT_TAG ?= $(shell git describe --tags --abbrev=0)
BIN_DIR = "dist/bin"

MDBX_PATH = "crates/storage/libmdbx-rs/mdbx-sys/libmdbx"
DB_TOOLS_DIR = "db-tools"
FULL_DB_TOOLS_DIR := $(shell pwd)/$(DB_TOOLS_DIR)/

BUILD_PATH = "target"

# List of features to use when building.
# Can be overriden via the environment.
# No jemalloc on Windows
ifeq ($(OS),Windows_NT)
    FEATURES ?=
else
    FEATURES ?= jemalloc
endif

# Cargo profile for builds. Default is for local builds, CI uses an override.
PROFILE ?= release

# Extra flags for Cargo
CARGO_INSTALL_EXTRA_FLAGS ?=

##@ Help

.PHONY: help
help: ## Display this help.
	@awk 'BEGIN {FS = ":.*##"; printf "Usage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_0-9-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

##@ Build

# Builds natively.
build-native-%:
	cargo build --target $* --features "$(FEATURES)" --profile "$(PROFILE)"

# The following commands use `cross` to build a cross-compile.
#
# These commands require that:
#
# - `cross` is installed (`cargo install cross`).
# - Docker is running.
# - The current user is in the `docker` group.
#
# The resulting binaries will be created in the `target/` directory.

# No jemalloc on Windows
build-x86_64-pc-windows-gnu: FEATURES := $(filter-out jemalloc jemalloc-prof,$(FEATURES))

# Note: The additional rustc compiler flags are for intrinsics needed by MDBX.
# See: https://github.com/cross-rs/cross/wiki/FAQ#undefined-reference-with-build-std
build-%:
	RUSTFLAGS="-C link-arg=-lgcc -Clink-arg=-static-libgcc" \
 		cross build --target $* --features "$(FEATURES)" --profile "$(PROFILE)"

# Unfortunately we can't easily use cross to build for Darwin because of licensing issues.
# If we wanted to, we would need to build a custom Docker image with the SDK available.
#
# Note: You must set `SDKROOT` and `MACOSX_DEPLOYMENT_TARGET`. These can be found using `xcrun`.
#
# `SDKROOT=$(xcrun -sdk macosx --show-sdk-path) MACOSX_DEPLOYMENT_TARGET=$(xcrun -sdk macosx --show-sdk-platform-version)`
build-x86_64-apple-darwin:
	$(MAKE) build-native-x86_64-apple-darwin
build-aarch64-apple-darwin:
	$(MAKE) build-native-aarch64-apple-darwin

# Create a `.tar.gz` containing a binary for a specific target.
define tarball_release_binary
	cp $(BUILD_PATH)/$(1)/$(PROFILE)/* $(BIN_DIR)/*
	cd $(BIN_DIR) && \
		tar -czf release-$(GIT_TAG)-$(1)$(2).tar.gz *
endef

# The current git tag will be used as the version in the output file names. You
# will likely need to use `git tag` and create a semver tag (e.g., `v0.2.3`).
#
# Note: This excludes macOS tarballs because of SDK licensing issues.
.PHONY: build-release-tarballs
build-release-tarballs: ## Create a series of `.tar.gz` files in the BIN_DIR directory, each containing a binary for a different target.
	[ -d $(BIN_DIR) ] || mkdir -p $(BIN_DIR)
	$(MAKE) build-x86_64-unknown-linux-gnu
	$(call tarball_release_binary,"x86_64-unknown-linux-gnu","")
	$(MAKE) build-aarch64-unknown-linux-gnu
	$(call tarball_release_binary,"aarch64-unknown-linux-gnu","")
	$(MAKE) build-x86_64-pc-windows-gnu
	$(call tarball_release_binary,"x86_64-pc-windows-gnu","")

##@ Test

UNIT_TEST_ARGS := --locked --workspace --all-features -E 'kind(lib)' -E 'kind(bin)' -E 'kind(proc-macro)'
COV_FILE := lcov.info

.PHONY: test-unit
test-unit: ## Run unit tests.
	cargo install cargo-nextest --locked
	cargo nextest run $(UNIT_TEST_ARGS)

.PHONY: cov-unit
cov-unit: ## Run unit tests with coverage.
	rm -f $(COV_FILE)
	cargo llvm-cov nextest --lcov --output-path $(COV_FILE) $(UNIT_TEST_ARGS)

.PHONY: cov-report-html
cov-report-html: cov-unit ## Generate a HTML coverage report and open it in the browser.
	cargo llvm-cov report --html
	open target/llvm-cov/html/index.html
