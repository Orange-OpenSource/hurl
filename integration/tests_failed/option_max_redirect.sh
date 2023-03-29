#!/bin/bash
set -Eeuo pipefail
hurl tests_failed/option_max_redirect.hurl
