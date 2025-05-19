#!/bin/bash
set -Eeuo pipefail

hurl  --ignore-asserts tests_ok/ignore_asserts/ignore_asserts.hurl
