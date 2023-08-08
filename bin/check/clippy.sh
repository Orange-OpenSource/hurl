#!/bin/bash
set -Eeuo pipefail
# https://rust-lang.github.io/rust-clippy/master/index.html
cargo clippy -- \
  --deny warnings \
  --deny clippy::empty_structs_with_brackets \
  --deny clippy::manual_string_new   


