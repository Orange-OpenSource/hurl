#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/proxy_option.hurl --verbose
