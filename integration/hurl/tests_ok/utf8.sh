#!/bin/bash
set -Eeuo pipefail
hurl --very-verbose tests_ok/utf8.hurl
