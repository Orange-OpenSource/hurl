#!/bin/bash
set -Eeuo pipefail

docker_file="$1"
hadolint --verbose --ignore DL3018 --ignore SC1091 "${docker_file}"

