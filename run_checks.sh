#!/usr/bin/env bash

echo "> Running tests..."
cargo test

echo "> Checking formatting..."
cargo fmt --check

echo "> Linting using Clippy..."
cargo clippy --all -- -D warnings

echo "> Running security audit..."
cargo deny check advisories
