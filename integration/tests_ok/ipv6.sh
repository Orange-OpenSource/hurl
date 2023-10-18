#!/bin/bash
set -Eeuo pipefail
echo "IPV4 test"
curl --ipv4 --head "https://google.com"
echo "IPV6 test"
curl --ipv6 --head "https://google.com"
