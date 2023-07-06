#!/bin/bash
set -Eeuo pipefail
rm -f build/result.tap
hurl --test --report-tap build/result.tap tests_ok/test.1.hurl tests_ok/test.2.hurl
hurl --test --report-tap build/result.tap tests_ok/test.3.hurl
cat build/result.tap
