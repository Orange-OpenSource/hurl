#!/bin/bash
set -Eeuo pipefail
hurl --glob 'tests_failed/many_glob_not_found.hurl' --glob 'does_not_exist/*.hurl'

