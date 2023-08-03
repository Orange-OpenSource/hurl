#!/bin/bash
set -Eeuo pipefail

cargo clippy -- \
  --deny warnings \
  --deny clippy::empty_structs_with_brackets # https://rust-lang.github.io/rust-clippy/master/index.html#/empty_structs_with_brackets


