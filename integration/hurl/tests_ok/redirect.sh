#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/redirect.hurl --verbose
