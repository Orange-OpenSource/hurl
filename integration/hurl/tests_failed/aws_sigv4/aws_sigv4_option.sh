#!/bin/bash
set -Eeuo pipefail


set +eo pipefail
# Check error message when curl/libcurl on this system does not support --aws-sigv4
# simply  ignore test if option is available on the system
# FIXME: remove this test once all integration test targets have aws-sigv4 support in libcurl
if curl --aws-sigv4 2>&1 | grep -qv 'option --aws-sigv4: is unknown'; then
    exit 255
fi

set -Eeuo pipefail
hurl --user someAccessKeyId:someSecretKey tests_ok/aws_sigv4/aws_sigv4_option.hurl
