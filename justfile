#!/usr/bin/env -S just --justfile
#
# To run this script, you must have installed the Just command runner. Execute:
# $ cargo install --locked just

# Default Rust toolchain
rust-toolchain := "stable"

#
# Setup the environment:
#

setup-cargo-binstall:
    cargo install --locked cargo-binstall

setup-cargo-hack: setup-cargo-binstall
    cargo binstall --no-confirm cargo-hack

setup-cargo-audit: setup-cargo-binstall
    cargo binstall --no-confirm cargo-audit

setup-cargo-machete: setup-cargo-binstall
    cargo binstall --no-confirm cargo-machete

setup: setup-cargo-binstall setup-cargo-hack setup-cargo-audit setup-cargo-machete
    git config pull.rebase true
    git config branch.autoSetupRebase always
    cargo binstall --no-confirm typos-cli
    cargo binstall --no-confirm cocogitto
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

cargo-fmt:
    cargo fmt --all

cargo-fmt-check:
    cargo fmt --check


#
# Misc recipes:
#

self-update: setup-cargo-binstall
    cargo binstall --no-confirm just

clean:
    cargo clean
