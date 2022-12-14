#!/bin/bash
set -e

# Get hurl source version
VERSION="$(grep '^version' packages/hurl/Cargo.toml | cut -f2 -d'"')"
echo "${VERSION}"
