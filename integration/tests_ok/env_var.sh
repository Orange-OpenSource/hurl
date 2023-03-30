#!/bin/bash
set -Eeuo pipefail
export HURL_name=Bob
hurl tests_ok/env_var.hurl
