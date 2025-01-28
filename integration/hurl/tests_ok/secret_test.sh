#!/bin/bash
set -Eeuo pipefail

hurl --test \
    --very-verbose \
    --secret a=secret1 \
    --secret b=secret2 \
    --secret c=12345678 \
    tests_ok/secret_test.hurl 2>build/secret_test.err

secrets=("secret1" "secret2" "secret3" "12345678")

file="build/secret_test.err"

for secret in "${secrets[@]}"; do
  if grep -q "$secret" "$file"; then
      echo "Secret <$secret> have leaked in $file"
      cat "$file"
      exit 1
  fi
done
