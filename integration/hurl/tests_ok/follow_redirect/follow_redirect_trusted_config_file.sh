#!/bin/bash
set -Eeuo pipefail

XDG_CONFIG_HOME=$(dirname "$0")/config_trusted
export XDG_CONFIG_HOME

hurl tests_ok/follow_redirect/follow_redirect_trusted.hurl