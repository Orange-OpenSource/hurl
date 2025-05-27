#!/bin/bash
set -Eeuo pipefail
hurl tests_ssl/error_options_pinnedpubkey.hurl
