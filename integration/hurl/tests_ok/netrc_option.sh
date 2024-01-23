#!/bin/bash
set -Eeuo pipefail
hurl --verbose tests_ok/netrc_option.hurl
