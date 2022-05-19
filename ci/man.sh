#!/bin/bash
set -e

rm -rf target/man
mkdir -p target/man
ci/gen_manpage.py docs/man/hurl.md > target/man/hurl.1
ci/gen_manpage.py docs/man/hurlfmt.md > target/man/hurlfmt.1

gzip -n -9 target/man/hurl.1
gzip -n -9 target/man/hurlfmt.1


