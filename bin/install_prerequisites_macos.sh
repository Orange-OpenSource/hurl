#!/bin/bash
set -Eeuo pipefail

echo "----- preinstalled curl version -----"
curl --version

echo "----- install prerequisite packages -----"
export HOMEBREW_NO_INSTALLED_DEPENDENTS_CHECK=true
brew uninstall --force --ignore-dependencies curl
brew update
brew install -s curl
brew link --force --overwrite curl
CURL_PATH="$(brew --prefix curl)/bin"
export CURL_PATH
echo "CURL_PATH=$CURL_PATH"
PATH="$CURL_PATH:$PATH"
export PATH
brew install bash expect pkg-config squid jq
sudo squid -k shutdown || true
sudo rm -v /dev/shm/squid*.shm >/dev/null 2>&1 || true

