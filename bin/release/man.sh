#!/bin/bash
set -e

rm -rf target/man
mkdir -p target/man
bin/release/gen_manpage.py docs/manual/hurl.md > target/man/hurl.1
bin/release/gen_manpage.py docs/manual/hurlfmt.md > target/man/hurlfmt.1

gzip -n -9 target/man/hurl.1
gzip -n -9 target/man/hurlfmt.1


