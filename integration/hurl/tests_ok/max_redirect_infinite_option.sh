#!/bin/bash
set -Eeuo pipefail

hurl --ipv4 tests_ok/max_redirect_infinite_option.hurl
