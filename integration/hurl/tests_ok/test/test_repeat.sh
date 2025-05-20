#!/bin/bash
set -Eeuo pipefail

hurl --test --repeat 100 tests_ok/test/test_repeat.hurl
