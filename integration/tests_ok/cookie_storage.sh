#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/cookie_storage.hurl --verbose
