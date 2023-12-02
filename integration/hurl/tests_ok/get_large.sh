#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/get_large.hurl --verbose
