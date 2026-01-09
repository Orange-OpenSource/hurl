#!/bin/bash
set -Eeuo pipefail

hurl --verbose --cookie tests_ok/cookie/cookie_file.cookies tests_ok/cookie/cookie_file.hurl
