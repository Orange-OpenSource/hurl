#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_ssl/error_options_pinnedpubkey.hurl
