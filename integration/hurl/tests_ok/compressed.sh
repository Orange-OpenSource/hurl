#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/compressed.hurl --compressed --verbose
