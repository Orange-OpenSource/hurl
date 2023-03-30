#!/bin/bash
set -Eeuo pipefail
hurl --test --glob "tests_ok/test.*.hurl"
