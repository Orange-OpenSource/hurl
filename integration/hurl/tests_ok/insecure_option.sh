#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/insecure_option.hurl --verbose
