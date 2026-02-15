#!/bin/bash
set -Eeuo pipefail
mkdir -p build/tmp

hurl --no-color --output build/tmp tests_ok/hello/hello.hurl
