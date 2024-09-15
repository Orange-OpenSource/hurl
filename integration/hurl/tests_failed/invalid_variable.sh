#!/bin/bash
set -Eeuo pipefail
set +e
hurl --variable newFoo=Bar --variable newUuid=123 tests_failed/invalid_variable.hurl

