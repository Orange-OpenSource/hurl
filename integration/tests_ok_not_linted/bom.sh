#!/bin/bash
set -Eeuo pipefail
hurl tests_ok_not_linted/bom.hurl --verbose
