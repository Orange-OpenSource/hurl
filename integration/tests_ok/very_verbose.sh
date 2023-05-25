#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/very_verbose.hurl --very-verbose --location
