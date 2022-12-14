#!/bin/bash
set -Eeuo pipefail

# Install packages
yum install -y bash python3-devel python3-pip
python3 -m pip install --upgrade pip --quiet

