#!/bin/bash
set -Eeuo pipefail
hurl --variable name=Bruce tests_ok/cookies.hurl
