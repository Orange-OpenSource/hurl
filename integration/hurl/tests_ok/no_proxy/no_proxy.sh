#!/bin/bash
set -Eeuo pipefail

# Disable proxy at the command-line
hurl --no-proxy 127.0.0.1 tests_ok/no_proxy/no_proxy.hurl

# Disable proxy from the config file
XDG_CONFIG_HOME=$(dirname "$0")/config
export XDG_CONFIG_HOME
hurl tests_ok/no_proxy/no_proxy.hurl
unset XDG_CONFIG_HOME

# Disable proxy from environment variable
no_proxy=127.0.0.1
export no_proxy
hurl --proxy localhost:3128 tests_ok/no_proxy/no_proxy.hurl
unset no_proxy
