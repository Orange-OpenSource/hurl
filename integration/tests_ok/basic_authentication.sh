#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/basic_authentication.hurl --user bob@email.com:secret --verbose
