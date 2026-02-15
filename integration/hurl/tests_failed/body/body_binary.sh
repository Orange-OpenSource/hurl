#!/bin/bash
set -Eeuo pipefail

hurl --no-color --continue-on-error tests_failed/body/body_binary.hurl
