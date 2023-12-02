#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/ignore_asserts.hurl --ignore-asserts
