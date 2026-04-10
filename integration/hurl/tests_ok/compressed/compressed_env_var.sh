#!/bin/bash
set -Eeuo pipefail

export HURL_COMPRESSED=1
hurl tests_ok/compressed/compressed.hurl
