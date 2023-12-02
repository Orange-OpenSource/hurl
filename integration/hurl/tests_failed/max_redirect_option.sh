#!/bin/bash
set -Eeuo pipefail
hurl tests_failed/max_redirect_option.hurl
