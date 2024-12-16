#!/bin/bash
set -Eeuo pipefail

hurl --test --very-verbose --secret a=foofoofoo --secret b=barbar --secret c=baz tests_ok/secret.hurl 2>build/secret_test.err

words=("foofoofoo" "barbar" "baz")

for word in "${words[@]}"; do
  if grep -q "$word" build/secret_test.err; then
      # Secrets have leaked!
      exit 1
  fi
done
