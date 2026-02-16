#!/bin/bash
set -Eeuo pipefail

rm -rf target/profile
rm -rf target/coverage
cargo clean

RUSTFLAGS="-Cinstrument-coverage"
export RUSTFLAGS
LLVM_PROFILE_FILE="$(pwd)/target/profile/test-integ-%p-%m.profraw"
export LLVM_PROFILE_FILE

cargo build
PATH=$(pwd)/target/debug:$PATH
export PATH
bin/test/test_integ.sh
grcov target/profile \
    --binary-path target/debug \
    --source-dir . \
    --output-types html \
    --branch \
    --ignore 'target/debug/build/clang-sys-*' \
    --ignore-not-existing \
    --excl-line "^ *\}$" \
    --output-path target/coverage

