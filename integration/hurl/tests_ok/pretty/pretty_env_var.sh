#!/bin/bash
set -Eeuo pipefail

export HURL_PRETTY=1
hurl tests_ok/pretty/pretty.hurl
