#!/bin/bash
set -Eeuo pipefail
rm -f build/result.tap

# test.2.hurl is KO but we want the script to continue until the end
set +eo pipefail
hurl --test --report-tap build/result.tap tests_ok/test.1.hurl tests_ok/test.2.hurl
hurl --test --report-tap build/result.tap tests_ok/test.3.hurl
set -Eeuo pipefail

cat build/result.tap
