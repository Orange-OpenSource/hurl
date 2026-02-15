#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_ssl/error_self_signed_certificate.hurl
