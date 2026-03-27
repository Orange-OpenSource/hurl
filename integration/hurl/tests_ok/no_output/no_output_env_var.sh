#!/bin/bash
set -Eeuo pipefail

export HURL_NO_OUTPUT=1
hurl tests_ok/no_output/no_output.hurl
