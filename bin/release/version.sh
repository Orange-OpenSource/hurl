#!/bin/sh
set -eu
echo VERSION="$(grep '^version' packages/hurl/Cargo.toml | cut -f2 -d'"')" >> "$GITHUB_ENV"
