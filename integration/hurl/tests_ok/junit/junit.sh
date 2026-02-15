#!/bin/bash
set -Eeuo pipefail
rm -f build/junit/result.xml

# test2 is KO but we want the script to continue until the end
set +eo pipefail
# FIXME: We simulate CI in order to disable progress bar (we don't have --no-progress-bar)
export CI=1
# We use --jobs 1 to force the standard error order to be test1 then test2.
hurl --no-color --test --jobs 1 --report-junit build/junit/result.xml tests_ok/junit/test.1.hurl tests_ok/junit/test.2.hurl
hurl --no-color --test --report-junit build/junit/result.xml tests_ok/junit/test.3.hurl
hurl --no-color --test --report-junit build/junit/result.xml tests_ok/junit/test.4.hurl
set -Eeuo pipefail

cat build/junit/result.xml
