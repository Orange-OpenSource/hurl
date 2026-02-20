#!/bin/bash
set -Eeuo pipefail

hurl --ipv6 tests_ok/ipv6/ipv6.hurl
