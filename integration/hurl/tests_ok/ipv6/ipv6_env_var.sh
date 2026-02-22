#!/bin/bash
set -Eeuo pipefail

export HURL_IPV6=1
hurl tests_ok/ipv6/ipv6.hurl
