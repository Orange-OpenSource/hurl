#!/bin/bash
set -Eeuo pipefail

hurl --verbose tests_ok/proxy_option.hurl
