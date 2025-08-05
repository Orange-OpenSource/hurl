#!/bin/bash
set -Eeuo pipefail

rm -rf build/secret

hurl --very-verbose \
    --secrets-file tests_ok/secret/secrets.env \
    --curl build/secret/curl.txt \
    --cookie-jar build/secret-cookies.txt \
    --report-html build/secret/report-html \
    --report-json build/secret/report-json \
    tests_ok/secret/secret.hurl

secrets=("secret1" "secret2" "secret3" "12345678" "secret-dynamic-0" "secret-dynamic-1" "secret-dynamic-2")

files=$(find build/secret/report-html/*.html \
  build/secret/report-html/**/*.html \
  build/secret/report-json/*.json \
  build/secret/curl.txt \
  build/secret-cookies.txt \
  tests_ok/secret/secret.err.pattern
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

