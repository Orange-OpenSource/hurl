#!/bin/bash
set -Eeuo pipefail

hurl --no-color --continue-on-error tests_failed/max_redirect/max_redirect_option.hurl
