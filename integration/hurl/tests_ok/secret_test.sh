#!/bin/bash
set -Eeuo pipefail

hurl --test \
    --very-verbose \
    --secret a=secret1 \
    --secret b=secret2 \
    --secret c=secret3 \
    tests_ok/secret.hurl 2>build/secret_test.err

secrets=("secret1" "secret2" "secret3")

file="build/secret_test.err"

for secret in "${secrets[@]}"; do
  if grep -q "$secret" "$file"; then
      echo "Secret <$secret> have leaked in $file"
      exit 1
  fi
done
