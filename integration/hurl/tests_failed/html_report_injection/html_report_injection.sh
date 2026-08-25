#!/bin/bash
set -Eeuo pipefail

rm -rf build/tests_failed_injection/report

# We test a Hurl file that triggers a runtime error and want to check that any HTML files
# in the report has a plain "<script>" tag.
set +eo pipefail
hurl --verbose --report-html build/tests_failed_injection/report tests_failed/html_report_injection/html_report_injection.hurl

grep -r '<script>' build/tests_failed_injection/report/store

exit 1
