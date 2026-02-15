#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_failed/predicate/predicate.hurl
