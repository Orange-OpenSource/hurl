#!/bin/bash
set -Eeuo pipefail

set +eo pipefail
hurl --version | grep Features | grep -q HTTP3
if [ $? -eq 1 ]; then
  exit 255
fi

# 13/08/2025: same error on Debian Trixie
# 17/02/2025: we deactivate this test on Arch Linux
# The GitHub runner are supporting HTTP/3 but we get this error:
# ```shell
#  expected: 0  actual:3
#  error: HTTP connection
#    --> tests_ok/http_version_3_option.hurl:6:6
#     |
#   6 | HEAD https://google.com
#     |      ^^^^^^^^^^^^^^^^^^ (95) HTTP/3 stream 0 reset by server
#     |
# ```
if [ -f /etc/os-release ]; then
    if grep -q 'NAME="Arch Linux"' /etc/os-release; then
        exit 255
    fi
    if grep -q 'NAME="Debian GNU/Linux"' /etc/os-release; then
        exit 255
    fi
fi
set -Eeuo pipefail

hurl tests_ok/http_version/http_version_3_option.hurl

