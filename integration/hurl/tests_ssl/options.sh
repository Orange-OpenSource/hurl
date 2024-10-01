#!/bin/bash
set -Eeuo pipefail
hurl tests_ssl/options.hurl --verbose
