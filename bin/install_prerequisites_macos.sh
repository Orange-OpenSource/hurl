#!/bin/bash
set -Eeuo pipefail

echo "----- install prerequisite packages -----"
export HOMEBREW_NO_INSTALLED_DEPENDENTS_CHECK=true
brew update
brew install bash expect squid jq
sudo squid -k shutdown || true
sudo rm -v /dev/shm/squid*.shm >/dev/null 2>&1 || true
