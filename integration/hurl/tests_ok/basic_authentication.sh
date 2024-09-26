#!/bin/bash
set -Eeuo pipefail
hurl --user bob@email.com:secret tests_ok/basic_authentication.hurl
