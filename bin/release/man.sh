#!/bin/bash
set -Eeuo pipefail

rm -rf target/man
mkdir -p target/man
bin/docs/build_man.py docs/manual/hurl.md > target/man/hurl.1
bin/docs/build_man.py docs/manual/hurlfmt.md > target/man/hurlfmt.1

gzip -n -9 target/man/hurl.1
gzip -n -9 target/man/hurlfmt.1

