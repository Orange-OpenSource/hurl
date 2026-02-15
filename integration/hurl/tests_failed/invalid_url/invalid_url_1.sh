#!/bin/bash
set -Eeuo pipefail

hurl --no-color --variable host=localhost:8000 tests_failed/invalid_url/invalid_url_1.hurl
