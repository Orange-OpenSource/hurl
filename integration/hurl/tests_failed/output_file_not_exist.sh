#!/bin/bash
set -Eeuo pipefail

hurl --output /foo/bar/baz tests_ok/hello.hurl
