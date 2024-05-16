#!/bin/bash
set -Eeuo pipefail
hurl --verbose tests_ok/follow_redirect_option.hurl
