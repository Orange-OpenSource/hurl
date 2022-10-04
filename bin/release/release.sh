#!/bin/sh
set -e

PATH="$HOME"/.cargo/bin:$PATH
export PATH
cargo build --release --verbose --locked

# Return PATH var to parent shell
package_dir="$(cd target/release ; pwd)"
echo "Run this if you want to use fresh builded hurl package:"
echo "  export PATH=$package_dir:$PATH"
