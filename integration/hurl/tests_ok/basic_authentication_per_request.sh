#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/basic_authentication_per_request.hurl
