#!/bin/bash
set -Eeuo pipefail
hurl tests_ok_not_linted/empty_section.hurl --verbose
