#!/bin/bash
set -Eeuo pipefail

hurl --include --pretty --color tests_ok/pretty/pretty.hurl
