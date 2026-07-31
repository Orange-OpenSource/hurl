#!/bin/bash
set -Eeuo pipefail

XDG_CONFIG_HOME=$(dirname "$0")/config
export XDG_CONFIG_HOME

hurl --ipv4 --location tests_ok/max_redirect/max_redirect_infinite.hurl
