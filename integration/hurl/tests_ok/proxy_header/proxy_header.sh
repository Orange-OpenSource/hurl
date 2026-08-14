#!/bin/bash
set -Eeuo pipefail

# Send proxy header Foo:Bar at the command-line
hurl --proxy-header Foo:Bar tests_ok/proxy_header/proxy_header.hurl

# Send proxy header Foo:Bar from config file
XDG_CONFIG_HOME=$(dirname "$0")/config
export XDG_CONFIG_HOME
hurl tests_ok/proxy_header/proxy_header.hurl
unset XDG_CONFIG_HOME

# Send proxy header Foo:Bar from environment variable
HURL_PROXY_HEADER="Foo:Bar"
export HURL_PROXY_HEADER
hurl tests_ok/proxy_header/proxy_header.hurl
unset HURL_PROXY_HEADER 

