#!/bin/bash
set -Eeuo pipefail
hurl tests_ssl/error_client_authentication_password.hurl
