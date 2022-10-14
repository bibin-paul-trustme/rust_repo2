#!/usr/bin/env sh

# Fail on errors
set -e -u

# Build the project
cargo build --verbose --all

# Test everything
cargo test --verbose --all

# Generate the pages
mkdir -p output
cargo run --release --bin=rustup-available-packages-web -- render -c config.yaml
# ln -fs x86_64-unknown-linux-gnu.html output/index.html
cp index.html output/
cp index.js output/
cp template.html output/
