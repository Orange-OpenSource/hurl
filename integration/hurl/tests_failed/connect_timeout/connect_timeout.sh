#!/bin/bash
set -Eeuo pipefail

hurl --no-color --connect-timeout 1 tests_failed/connect_timeout/connect_timeout.hurl
