#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/delay.hurl --delay 1000 --verbose
