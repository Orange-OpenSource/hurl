#!/bin/bash
set -Eeuo pipefail
rm -f build/fileroot.bin
hurl --file-root build/ tests_ok/fileroot.hurl
