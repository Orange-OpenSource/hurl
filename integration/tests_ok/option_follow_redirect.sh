#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/option_follow_redirect.hurl --verbose
