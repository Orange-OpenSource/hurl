#!/bin/bash
set -Eeuo pipefail

hurl --no-header foo --no-header '' tests_failed/no_header/no_header.hurl
