#!/bin/bash
set -Eeuo pipefail

set +eo pipefail
# if curl/libcurl on this system does not support --aws-sigv4
# simply ignore test
# FIXME: remove this workaround once all integration test targets have aws-sigv4 support in libcurl
if curl --aws-sigv4 2>&1 | grep -q 'option --aws-sigv4: is unknown'; then
    exit 255
fi
set -Eeuo pipefail

hurl tests_ok/aws_sigv4_option.hurl
