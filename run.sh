#!/bin/bash
export CARGO_HOME="$(pwd)/.cargo"
export RUSTUP_HOME="$(pwd)/.rustup"
export CARGO_TARGET_DIR="$(pwd)/target"

cargo run "$@"