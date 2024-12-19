#!/bin/bash
set -Eeuo pipefail

hurl --very-verbose \
    --secret a=secret1 \
    --secret b=secret2 \
    --secret c=secret3 \
    --report-html build/secret \
    tests_ok/secret.hurl

secrets=("secret1" "secret2" "secret3")

files=$(find build/secret/*.html build/secret/**/*.html)

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

