#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/option_insecure.hurl --verbose
