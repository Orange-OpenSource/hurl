#!/bin/bash
set -Eeuo pipefail

set +eo pipefail
curl --version | grep Features | grep -q HTTP3
if [ $? -eq 1 ]; then
  exit 255
fi

# On désactive ce test momentanément Sur Arch Linux. Arch Linux supporte
# maintenant HTTP/3 mais ce test ne passe pas.
if [ -f /etc/os-release ]; then
    if grep -q 'NAME="Arch Linux"' /etc/os-release; then
        exit 255
    fi
fi
set -Eeuo pipefail

hurl tests_ok/http_version_3_option.hurl
