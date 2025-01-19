#!/bin/bash
set -Eeuo pipefail

rm -rf build/report/json

# test.2.hurl is KO but we want the script to continue until the end
set +eo pipefail
hurl --test --report-json build/report/json tests_ok/test.1.hurl tests_ok/test.2.hurl
hurl --test --report-json build/report/json tests_ok/test.3.hurl
set -Eeuo pipefail

cat build/report/json/report.json
