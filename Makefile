###############################
# Common defaults/definitions #
###############################

comma := ,

# Checks two given strings for equality.
eq = $(if $(or $(1),$(2)),$(and $(findstring $(1),$(2)),\
                                $(findstring $(2),$(1))),1)
# Reverses given list.
reverse = $(if $(1),$(call reverse,$(wordlist 2,$(words $(1)),$(1)))) \
          $(firstword $(1))
# Makes given string usable in URL.
# Analogue of slugify() function from GitLab:
# https://gitlab.com/gitlab-org/gitlab-foss/blob/master/lib/gitlab/utils.rb
slugify = $(strip $(shell echo $(2) | tr [:upper:] [:lower:] \
                                    | tr -c [:alnum:] - \
                                    | cut -c 1-$(1) \
                                    | sed -e 's/^-*//' -e 's/-*$$//'))




######################
# Project parameters #
######################

IMAGE_NAME := $(strip $(shell grep 'COMPOSE_IMAGE_NAME=' .env | cut -d '=' -f2))

RELEASE_BRANCH := release
MAINLINE_BRANCH := master
CURRENT_BRANCH := $(strip $(if $(call eq,$(CI_SERVER),yes),\
	$(CI_COMMIT_REF_NAME),$(shell git branch | grep \* | cut -d ' ' -f2)))

CURRENT_OS := $(strip $(or $(os),\
	$(if $(call eq,$(OS),Windows_NT),windows,\
	$(if $(call eq,$(shell uname -s),Darwin),macos,linux))))

VERSION ?= $(strip $(shell grep -m1 'version = "' Cargo.toml | cut -d '"' -f2))
CARGO_HOME ?= $(strip $(shell dirname $$(dirname $$(which cargo))))
NODE_VER ?= $(strip $(shell grep 'NODE_VER: ' .gitlab-ci.yml \
	| cut -d ':' -f2 | cut -d "'" -f2))
RUST_VER ?= $(strip $(shell grep 'RUST_VER: ' .gitlab-ci.yml \
	| cut -d ':' -f2 | cut -d "'" -f2))
RUST_NIGHTLY_VER ?= $(strip $(shell grep 'RUST_NIGHTLY_VER: ' .gitlab-ci.yml \
	| cut -d ':' -f2 | cut -d "'" -f2))

PLATFORMS := admin collab social




###########
# Aliases #
###########

all: fmt lint test.unit


# Resolve all project dependencies.
#
# Usage:
#	make deps


fmt: cargo.fmt


lint: cargo.lint


# Run all project tests.
#
# Usage:
#	make test

test: test.unit




# Build project with Cargo.
#
# Usage:
#	make cargo.build [debug=(yes|no)]
#	                 [env=<env-vars>]
#	                 [background=(no|yes)]
#	                 [dockerized=(no|yes)]

cargo.build:
	@make cargo \
		cmd='cargo build --bin backend $(if $(call eq,$(debug),no),--release,)'\
		env='$(cargo-inventory-fix-env) $(env)' \
		background='$(background)' \
		dockerized='$(dockerized)'


# Format Rust sources with rustfmt.
#
# Usage:
#	make cargo.fmt [check=(no|yes)]
#	               [dockerized=(no|yes)]

cargo.fmt:
	cargo +nightly fmt --all $(if $(call eq,$(check),yes),-- --check,)


# Lint Rust sources with clippy.
#
# Usage:
#	make cargo.lint

cargo.lint:
	cargo clippy --workspace -- -D clippy::pedantic -D warnings


# Run Rust unit tests of project.
#
# Usage:
#	make test.unit

test.unit:
	cargo test -all


###################
# rustup commands #
###################

# Install nightly Rust toolchain of concrete date via rustup and link it
# as a default nightly Rust toolchain.
#
# The installer script is updated automatically to the latest version every day.
# For manual update use 'update-installer=yes' command option.
#
# Usage:
#	make rustup.nightly date=<YYYY-MM-DD>
#	                    [update-installer=(no|yes)]

rustup.nightly:
ifeq ($(update-installer),yes)
	$(call rustup.nightly.download)
else
ifeq ($(wildcard $(HOME)/.rustup/instrumentisto-nightly.sh),)
	$(call rustup.nightly.download)
else
ifneq ($(shell find $(HOME)/.rustup/instrumentisto-nightly.sh -mmin +1440),)
	$(call rustup.nightly.download)
endif
endif
endif
	@RUSTUP_NIGHTLY_DATE=$(date) \
	 $(if $(call eq,$(force),yes),RUSTUP_FORCE=yes,) \
	$(HOME)/.rustup/instrumentisto-nightly.sh
define rustup.nightly.download
	$()
	@mkdir -p $(HOME)/.rustup/
	@rm -f $(HOME)/.rustup/instrumentisto-nightly.sh
	curl -fL -o $(HOME)/.rustup/instrumentisto-nightly.sh \
		https://raw.githubusercontent.com/instrumentisto/toolchain/master/rustup/nightly.sh
	@chmod +x $(HOME)/.rustup/instrumentisto-nightly.sh
endef

