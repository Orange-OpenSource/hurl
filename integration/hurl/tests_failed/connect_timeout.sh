#!/bin/bash
set -Eeuo pipefail
hurl tests_failed/connect_timeout.hurl --connect-timeout 1
