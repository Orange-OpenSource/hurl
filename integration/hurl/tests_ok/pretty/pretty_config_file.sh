#!/bin/bash
set -Eeuo pipefail

XDG_CONFIG_HOME=$(dirname "$0")
export XDG_CONFIG_HOME
hurl tests_ok/pretty/pretty.hurl
