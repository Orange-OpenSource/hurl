#!/bin/bash
set -Eeuo pipefail
hurl ssl/error_self_signed_certificate.hurl
