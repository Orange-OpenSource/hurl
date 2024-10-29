#!/bin/bash
set -Eeuo pipefail

cat <<END | cargo valgrind run -p hurl -- --test
GET https://unpkg.com/vue@3.4.27/dist/vue.global.prod.js
HTTP 200
END
