#!/bin/bash
set -Eeuo pipefail

hurl --no-color --output /foo/bar/baz tests_ok/hello/hello.hurl
