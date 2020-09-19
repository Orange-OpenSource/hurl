#!/bin/bash
set -e

rm -rf target/man
mkdir -p target/man
cp docs/hurl.1 target/man
cp docs/hurlfmt.1 target/man
gzip target/man/hurl.1
gzip target/man/hurlfmt.1


