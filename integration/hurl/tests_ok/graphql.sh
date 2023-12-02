#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/graphql.hurl --verbose
