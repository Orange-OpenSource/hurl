#!/bin/bash
set -Eeuo pipefail

mkdir -p build
echo "GET     http://localhost:8000/hello" >build/test.hurl
hurlfmt --in-place build/test.hurl
cat build/test.hurl
