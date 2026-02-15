#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_failed/invalid_url/invalid_url.hurl
