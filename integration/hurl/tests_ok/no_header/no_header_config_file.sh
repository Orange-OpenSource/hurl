#!/bin/bash
set -Eeuo pipefail

XDG_CONFIG_HOME=$(dirname "$0")
export XDG_CONFIG_HOME
hurl tests_ok/no_header/no_header.hurl
