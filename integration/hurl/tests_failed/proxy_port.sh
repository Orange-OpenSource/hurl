#!/bin/bash
set -Eeuo pipefail

hurl --proxy localhost:1111 tests_ok/hello/hello.hurl
