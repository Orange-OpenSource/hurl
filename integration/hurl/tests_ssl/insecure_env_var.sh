#!/bin/bash
set -Eeuo pipefail

export HURL_INSECURE=1
hurl tests_ssl/insecure.hurl
