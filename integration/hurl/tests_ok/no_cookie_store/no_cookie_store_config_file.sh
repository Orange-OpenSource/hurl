#!/bin/bash
set -Eeuo pipefail

XDG_CONFIG_HOME=$(dirname "$0")
export XDG_CONFIG_HOME
hurl tests_ok/no_cookie_store/no_cookie_store.hurl
