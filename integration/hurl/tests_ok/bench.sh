#!/bin/bash
set -Eeuo pipefail

hurl --ipv4 tests_ok/bench.hurl
