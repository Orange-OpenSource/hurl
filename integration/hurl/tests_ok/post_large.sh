#!/bin/bash
set -Eeuo pipefail
dd if=/dev/zero of=build/post_large.bin bs=15728640 count=1 status=none
hurl --verbose --file-root build/ tests_ok/post_large.hurl
