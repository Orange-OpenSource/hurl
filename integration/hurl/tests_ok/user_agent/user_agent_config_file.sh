#!/bin/bash
set -Eeuo pipefail

XDG_CONFIG_HOME=$(dirname "$0")/config
export XDG_CONFIG_HOME

hurl tests_ok/user_agent/user_agent.hurl
