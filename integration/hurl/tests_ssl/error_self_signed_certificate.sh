#!/bin/bash
set -Eeuo pipefail
hurl tests_ssl/error_self_signed_certificate.hurl
