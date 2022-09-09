#!/usr/bin/env bash
cargo +nightly build --verbose
cargo +nightly build --no-default-features --features ron-config --verbose
cargo clippy --all-targets
cargo clippy --no-default-features --features ron-config --all-targets
cargo fmt -- --check
