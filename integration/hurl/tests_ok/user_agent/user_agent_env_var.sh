#!/bin/bash
set -Eeuo pipefail

export HURL_USER_AGENT="Mozilla/5.0 A"
hurl tests_ok/user_agent/user_agent.hurl
