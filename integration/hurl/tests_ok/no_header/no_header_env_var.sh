#!/bin/bash
set -Eeuo pipefail

export HURL_NO_HEADER="user-agent |Accept"
hurl tests_ok/no_header/no_header.hurl
