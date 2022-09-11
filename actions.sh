#!/usr/bin/env bash
cargo +nightly build --verbose
cargo clippy --all-targets
cargo fmt -- --check
