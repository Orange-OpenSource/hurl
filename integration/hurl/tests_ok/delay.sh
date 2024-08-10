#!/bin/bash
set -Eeuo pipefail
hurl --delay 1000 tests_ok/delay.hurl
hurl --delay 1s tests_ok/delay.hurl
