#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/cookie_file.hurl --cookie tests_ok/cookie_file.cookies --verbose
