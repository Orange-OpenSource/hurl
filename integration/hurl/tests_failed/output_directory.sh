#!/bin/bash
set -Eeuo pipefail
mkdir -p build/tmp

hurl --output build/tmp tests_ok/hello.hurl
