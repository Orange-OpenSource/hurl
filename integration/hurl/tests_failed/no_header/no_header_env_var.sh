#!/bin/bash
set -Eeuo pipefail

export HURL_NO_HEADER="foo|"
hurl tests_failed/no_header/no_header.hurl
