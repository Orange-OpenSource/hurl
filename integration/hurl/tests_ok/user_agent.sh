#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/user_agent.hurl --user-agent "Mozilla/5.0 A"
