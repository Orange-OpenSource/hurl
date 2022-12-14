#!/bin/bash
set -Eeuo pipefail

docker_file="$1"
version="2.12.0"
wget --quiet --output-document /tmp/hadolint "https://github.com/hadolint/hadolint/releases/download/v${version}/hadolint-Linux-x86_64"
chmod +x /tmp/hadolint
/tmp/hadolint --verbose "${docker_file}"

