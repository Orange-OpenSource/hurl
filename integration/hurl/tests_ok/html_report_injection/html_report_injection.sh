#!/bin/bash
set -Eeuo pipefail

rm -rf build/tests_ok_injection/report

hurl --report-html build/tests_ok_injection/report tests_ok/html_report_injection/html_report_injection.hurl

if grep -r '<script>' build/tests_ok_injection/report/store; then
  echo "Found unescaped <script> in HTML report"
  exit 1
fi
