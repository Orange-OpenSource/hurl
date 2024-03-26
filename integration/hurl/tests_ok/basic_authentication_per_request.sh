#!/bin/bash
set -Eeuo pipefail
hurl --verbose tests_ok/basic_authentication_per_request.hurl
