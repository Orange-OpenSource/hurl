#!/bin/bash
set -Eeuo pipefail

echo "----- install prerequisite packages -----"
export HOMEBREW_NO_INSTALLED_DEPENDENTS_CHECK=true
brew update
brew install bash mitmproxy jq
