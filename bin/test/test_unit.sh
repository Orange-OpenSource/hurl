#!/bin/sh
set -e
echo "----- unit tests  -----"
PATH="$HOME"/.cargo/bin:$PATH
export PATH
cargo test --release --features strict --tests

# Return PATH var to parent shell
package_dir="$(cd target/release ; pwd)"
echo "Run this if you want to use fresh builded hurl package:"
echo "  export PATH=$package_dir:$PATH"
