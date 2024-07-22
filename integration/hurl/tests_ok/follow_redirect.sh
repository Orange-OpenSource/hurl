#!/bin/bash
set -Eeuo pipefail
hurl --location tests_ok/follow_redirect.hurl
