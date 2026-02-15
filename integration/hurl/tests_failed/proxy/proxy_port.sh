#!/bin/bash
set -Eeuo pipefail

hurl --no-color --proxy localhost:1111 tests_failed/proxy/proxy.hurl
