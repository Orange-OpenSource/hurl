#!/bin/sh
set -eu
echo "----- install prerequisite packages -----"
yum install -y procps gcc libxml2-devel openssl-devel libcurl-devel python3-devel python3-pip
python3 -m pip install --upgrade pip --quiet
