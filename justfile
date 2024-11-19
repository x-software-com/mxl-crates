#!/usr/bin/env -S just --justfile
#
# To run this script, you must have installed the Just command runner. Execute:
# $ cargo install --locked just

# Default Rust toolchain
rust-toolchain := "stable"

#
# Setup the environment:
#

setup-cargo-hack:
    cargo install --locked cargo-hack

setup-cargo-audit:
    cargo install --locked cargo-audit

setup: setup-cargo-hack setup-cargo-audit
    git config pull.rebase true
    git config branch.autoSetupRebase always
    cargo install --locked typos-cli
    cargo install --locked cocogitto
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

cargo-fmt:
    cargo fmt --all

cargo-fmt-check:
    cargo fmt --check


#
# Misc recipes:
#

self-update:
    cargo install --locked just

clean:
    cargo clean

#
# Docker image for local testing:
#

docker-tag := "mxl-crates-test"

docker-build-no-cache:
    docker build --no-cache -t {{docker-tag}} -f docker/Dockerfile docker

docker-build:
    docker build -t {{docker-tag}} -f docker/Dockerfile docker

docker-run: docker-build
    #!/usr/bin/env bash
    set -e
    # Get parent directory as the mountpoint for the volume.
    MOUNT_DIR="$(dirname "$(pwd)")"
    docker run --privileged=true -it --rm \
        -v ${HOME}/.ssh:/root/.ssh \
        -v ${MOUNT_DIR}:${MOUNT_DIR} \
        --workdir $(pwd) {{docker-tag}} bash
