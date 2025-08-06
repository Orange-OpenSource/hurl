#!/bin/bash
set -Eeuo pipefail

hurl --proxy localhost:1111 tests_failed/proxy/proxy.hurl
