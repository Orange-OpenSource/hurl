#!/bin/bash
set -Eeuo pipefail
rm -f build/result.xml
hurl --test --report-junit build/result.xml tests_ok/test.1.hurl tests_ok/test.2.hurl
hurl --test --report-junit build/result.xml tests_ok/test.3.hurl
cat build/result.xml
