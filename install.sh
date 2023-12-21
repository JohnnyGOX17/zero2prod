#!/usr/bin/env bash

rustup update
echo "> Installing code coverage"
cargo install cargo-tarpaulin
echo "> Check crates for security vulnerabilities"
cargo install --locked cargo-deny && cargo deny init
echo "> Installing cargo expand"
cargo install cargo-expand
echo "> Installing Rust linting and formatter"
rustup component add clippy rustfmt

GIT_DIR=$(git rev-parse --git-dir)

echo "Installing pre-commit git hook..."
ln -s ../../pre-commit "$GIT_DIR/hooks/pre-commit"
