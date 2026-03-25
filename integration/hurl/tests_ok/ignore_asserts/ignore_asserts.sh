#!/bin/bash
set -Eeuo pipefail

hurl --no-assert tests_ok/ignore_asserts/ignore_asserts.hurl
