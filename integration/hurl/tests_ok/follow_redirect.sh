#!/bin/bash
set -Eeuo pipefail
hurl --location --verbose tests_ok/follow_redirect.hurl
