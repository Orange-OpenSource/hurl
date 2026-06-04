#!/bin/bash
set -Eeuo pipefail

XDG_CONFIG_HOME=$(dirname "$0")/config
export XDG_CONFIG_HOME

hurl tests_ok/ipv6/ipv6.hurl
