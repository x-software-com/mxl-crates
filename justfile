#!/usr/bin/env -S just --justfile
#
# To run this script, you must have installed the Just command runner. Execute:
# $ cargo install --locked just

# Default Rust toolchain
rust-toolchain := "stable"

#
# Setup the environment:
#

setup-cargo-edit:
    cargo install cargo-edit

setup-cargo-upgrades: setup-cargo-edit
    cargo install cargo-upgrades

setup-cargo-hack:
    cargo install cargo-hack

setup-cargo-audit:
    cargo install cargo-audit

setup-cargo-machete:
    cargo install cargo-machete

setup-cargo-deny:
    cargo install cargo-deny

setup: setup-cargo-upgrades setup-cargo-hack setup-cargo-audit setup-cargo-machete setup-cargo-deny
    git config pull.rebase true
    git config branch.autoSetupRebase always
    cargo install typos-cli
    cargo install cocogitto
    cog install-hook --overwrite commit-msg
    @echo "Done"

#
# Recipes for test and linting:
#

test-options := ""

test rust-toolchain=rust-toolchain:
    cargo +{{rust-toolchain}} test --no-fail-fast --workspace --all-features --all-targets -- {{test-options}}

test-verbose rust-toolchain=rust-toolchain:
    just --justfile {{justfile()}} test-options="--nocapture" test {{rust-toolchain}}

ci-test rust-toolchain=rust-toolchain:
    xvfb-run --auto-servernum --server-args="-screen 0 800x600x24" just --justfile {{justfile()}} test-verbose {{rust-toolchain}}

hack rust-toolchain=rust-toolchain: setup-cargo-hack
    cargo +{{rust-toolchain}} hack --feature-powerset --no-dev-deps check

clippy rust-toolchain=rust-toolchain:
    cargo +{{rust-toolchain}} clippy --quiet --release --all-targets --all-features

audit: setup-cargo-audit
    cargo audit

machete: setup-cargo-machete
    cargo machete --with-metadata

upgrades: setup-cargo-upgrades
    cargo upgrades

deny-licenses: setup-cargo-deny
    cargo deny --all-features check licenses

deny: setup-cargo-deny
    cargo deny --all-features check

cargo-fmt:
    cargo fmt --all

cargo-fmt-check:
    cargo fmt --check


#
# Misc recipes:
#

self-update:
    cargo install just

clean:
    cargo clean

clean-build: clean
    find . -name "config.rs" -delete
    find . | grep -E "(/__pycache__$|\.pyc$|\.pyo$)" | xargs rm -rf

clean-cache: clean-build
    rm -rf .cargo-cache
    @echo "Cleaned all cache directories"
