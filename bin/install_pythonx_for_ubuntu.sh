#!/bin/bash
set -Eeuo pipefail

python_version="${1:-11}"
sudo apt-get install -y software-properties-common
grep -R deadsnakes /etc/apt/ 2>/dev/null 2>&1 || sudo add-apt-repository -y ppa:deadsnakes/ppa
sudo DEBIAN_FRONTEND=noninteractive apt-get -y install \
    python3."${python_version}" \
    python3."${python_version}"-dev \
    python3."${python_version}"-venv
sudo update-alternatives --install /usr/bin/python3 python3 /usr/bin/python3."${python_version}" 1 || true
sudo update-alternatives --install /usr/bin/python python /usr/bin/python3."${python_version}" 1 || true

