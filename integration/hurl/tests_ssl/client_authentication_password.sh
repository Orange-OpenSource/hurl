#!/bin/bash
set -Eeuo pipefail
hurl tests_ssl/client_authentication_password.hurl
