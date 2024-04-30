#!/bin/bash
set -Eeuo pipefail
rm -f build/result.xml

# test.2.hurl is KO but we want the script to continue until the end
set +eo pipefail
# We use --jobs 1 to force the standard error order to be test1 then test2.
hurl --test --jobs 1 --report-junit build/result.xml tests_ok/test.1.hurl tests_ok/test.2.hurl
hurl --test --report-junit build/result.xml tests_ok/test.3.hurl
set -Eeuo pipefail

cat build/result.xml
