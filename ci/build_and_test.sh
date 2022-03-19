#!/bin/bash
set -e

echo "----- build -----"
# shellcheck source=/dev/null
source ~/.cargo/env
cargo build --verbose

ci/test_prerequisites.sh

echo "----- unit tests  -----"
cargo test --features strict --tests

echo "----- integration tests -----"
export PATH="$PWD/target/debug:$PATH"
cd integration || exit
./integration.py
