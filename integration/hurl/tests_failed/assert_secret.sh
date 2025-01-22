#!/bin/bash
set -Eeuo pipefail

rm -rf build/assert_secret

# We want to check leaks and do not stop at the first error
set +euo pipefail

hurl --secret name1=Alice \
    --secret name2=Bob \
    --error-format long \
    --report-html build/assert_secret/report-html \
    --report-json build/assert_secret/report-json \
    --report-junit build/assert_secret/report-junit/junit.xml \
    tests_failed/assert_secret.hurl

ret=$?

secrets=("Alice" "Bob")

files=$(find build/assert_secret/report-html/*.html \
  build/assert_secret/report-html/**/*.html \
  build/assert_secret/report-json/*.json \
  build/assert_secret/report-junit/junit.xml \
  tests_failed/assert_secret.err.pattern)

for secret in "${secrets[@]}"; do
  for file in $files; do
    # Don't search leaks in sources
    if [[ "$file" == *source.html ]]; then
      continue
    fi
    if grep -q "$secret" "$file"; then
        echo "Secret <$secret> have leaked in $file"
        exit 1
    fi
  done
done

# We use the exit code of the Hurl command
exit $ret
