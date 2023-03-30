#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/follow_redirect.hurl --location --verbose
