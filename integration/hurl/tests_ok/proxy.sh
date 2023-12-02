#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/proxy.hurl --proxy localhost:3128 --verbose
hurl tests_ok/proxy.hurl --proxy 127.0.0.1:3128 --verbose

