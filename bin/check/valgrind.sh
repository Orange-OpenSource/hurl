#!/bin/bash
set -Eeuo pipefail

valgrind --version
cargo-valgrind --help

# Disable valgrind for the moment, see <https://github.com/jfrimmel/cargo-valgrind/issues/131>
#cat <<END | cargo valgrind run -p hurl -- --test
#GET https://unpkg.com/vue@3.4.27/dist/vue.global.prod.js
#HTTP 200
#END
