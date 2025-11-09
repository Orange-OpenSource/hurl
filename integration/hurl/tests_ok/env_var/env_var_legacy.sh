#!/bin/bash
set -Eeuo pipefail

export HURL_name=Bob
hurl tests_ok/env_var/env_var.hurl
