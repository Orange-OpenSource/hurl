#!/bin/bash
set -Eeuo pipefail
hurl --delay 1000 tests_ok/delay.hurl
