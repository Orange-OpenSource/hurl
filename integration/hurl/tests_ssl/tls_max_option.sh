#!/bin/bash
set -Eeuo pipefail

hurl --variable ver=1.2 --verbose tests_ssl/tls_max_option.hurl
