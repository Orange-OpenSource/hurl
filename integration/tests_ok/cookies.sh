#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/cookies.hurl --variable name=Bruce --verbose
