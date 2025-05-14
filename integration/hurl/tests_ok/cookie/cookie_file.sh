#!/bin/bash
set -Eeuo pipefail

hurl --cookie tests_ok/cookie/cookie_file.cookies --verbose tests_ok/cookie/cookie_file.hurl
