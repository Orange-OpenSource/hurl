#!/bin/bash
set -Eeuo pipefail
hurl --verbose tests_ok/post_large.hurl
