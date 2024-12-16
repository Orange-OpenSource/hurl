#!/bin/bash
set -Eeuo pipefail

hurl --very-verbose \
    --secret a=foofoofoo \
    --secret b=barbar \
    --secret c=baz \
    tests_ok/secret.hurl
