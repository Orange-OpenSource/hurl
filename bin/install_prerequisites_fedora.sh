#!/bin/bash
set -Eeuo pipefail

echo "----- install prerequisite packages -----"
yum install -y bash procps gcc libxml2-devel openssl-devel libcurl-devel python3-devel python3-pip nc
python3 -m pip install --upgrade pip --quiet

