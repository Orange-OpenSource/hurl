#!/bin/bash
set -Eeuo pipefail

hurl --user-agent "Mozilla/5.0 A" tests_ok/user_agent/user_agent.hurl
