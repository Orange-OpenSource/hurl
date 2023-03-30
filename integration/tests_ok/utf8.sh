#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/utf8.hurl --verbose
