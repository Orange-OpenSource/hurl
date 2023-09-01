#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/delay.hurl --verbose
