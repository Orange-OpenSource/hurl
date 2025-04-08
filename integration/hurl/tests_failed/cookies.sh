#!/bin/bash
set -Eeuo pipefail
hurl tests_failed/cookies.hurl
