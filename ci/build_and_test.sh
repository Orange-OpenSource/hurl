#!/bin/bash
set -e

echo "----- build -----"
# Directive for ShellCheck SC1090:
# shellcheck source=/dev/null
source ~/.cargo/env
cargo build --release --verbose --locked
target/release/hurl --version
curl --version

ci/test_prerequisites.sh

echo "----- unit tests  -----"
cargo test --features strict --tests

echo "----- integration tests -----"
export PATH="$PWD/target/debug:$PATH"
cd integration || exit
./integration.py
