#!/bin/bash
set -Eeuo pipefail

hurl --continue-on-error tests_failed/max_redirect_option.hurl
