#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/delay_option.hurl --verbose
