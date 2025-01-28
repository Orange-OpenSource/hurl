#!/bin/bash
set -Eeuo pipefail

rm -rf build/secret

hurl --very-verbose \
    --secret a=secret1 \
    --secret b=secret2 \
    --secret c=12345678 \
    --curl build/secret/curl.txt \
    --cookie-jar build/secret-cookies.txt \
    --report-html build/secret/report-html \
    --report-json build/secret/report-json \
    tests_ok/secret.hurl

secrets=("secret1" "secret2" "secret3" "12345678")

files=$(find build/secret/report-html/*.html \
  build/secret/report-html/**/*.html \
  build/secret/report-json/*.json \
  build/secret/curl.txt \
  build/secret-cookies.txt \
  tests_ok/secret.err.pattern
)

for secret in "${secrets[@]}"; do
  for file in $files; do
    # Don't search leaks in sources
    if [[ "$file" == *source.html ]]; then
      continue
    fi
    if grep -q "$secret" "$file"; then
        echo "Secret <$secret> have leaked in $file"
        cat "$file"
        exit 1
    fi
  done
done

