#!/bin/bash
set -Eeuo pipefail

hurl --pretty --include tests_ok/pretty_print/pretty_print_with_headers.hurl
