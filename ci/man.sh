#!/bin/bash
set -e

rm -rf target/man
mkdir -p target/man
cp docs/hurl.1 target/man
cp docs/hurlfmt.1 target/man
gzip -n -9 target/man/hurl.1
gzip -n -9 target/man/hurlfmt.1


