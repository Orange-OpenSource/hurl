#!/bin/bash
set -Eeuo pipefail

export HURL_SECRET_a=secret1
export HURL_SECRET_b=secret2
export HURL_SECRET_c=12345678

hurl \
    --very-verbose \
    tests_ok/secret/secret_from_env.hurl 2>build/secret_from_env.err

secrets=("secret1" "secret2" "secret3" "12345678")

file="build/secret_from_env.err"

for secret in "${secrets[@]}"; do
  if grep -q "$secret" "$file"; then
      echo "Secret <$secret> have leaked in $file"
      cat "$file"
      exit 1
  fi
done
