#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_failed_not_linted/tab.hurl
