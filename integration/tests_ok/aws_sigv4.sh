#!/bin/bash
set -Eeuo pipefail
hurl --user someAccessKeyId:someSecretKey tests_ok/aws_sigv4.hurl --verbose
