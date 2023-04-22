#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/import_curl.out >/dev/null  # Validate expected file
hurlfmt --in curl tests_ok/import_curl.in
