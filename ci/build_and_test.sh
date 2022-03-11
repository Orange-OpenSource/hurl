#!/bin/bash
set -e

echo "----- build -----"
# shellcheck source=/dev/null
source ~/.cargo/env
cargo build --verbose

ci/test_prerequisites.sh

# current bug with curl to be fixed
# https://github.com/curl/curl/issues/8559
if test -f /etc/arch-release; then
   exit 0
fi

echo "----- unit tests  -----"
cargo test --features strict --tests

echo "----- integration tests -----"
export PATH="$PWD/target/debug:$PATH"
cd integration || exit
./integration.py
