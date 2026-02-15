#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_ssl/error_client_authentication_password.hurl
