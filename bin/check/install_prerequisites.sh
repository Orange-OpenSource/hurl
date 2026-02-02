#!/bin/bash
set -Eeuo pipefail

echo "# Install Python packages..."
python3 -m pip install \
    --requirement bin/requirements-frozen.txt \
    zizmor \
    requests

echo "# Install Rust packages..."
cargo install cargo-semver-checks --locked
# cargo-valgrind 2.4.0 lacks suppression files for Rust 1.93 (see https://github.com/Orange-OpenSource/hurl/issues/4738)
cargo install --git https://github.com/jfrimmel/cargo-valgrind cargo-valgrind

echo "# Install system packages..."
sudo apt-get update
sudo apt-get install -y \
    libxml2-utils \
    valgrind \
    zsh \
    fish \
    powershell

echo "# Install Rust toolchain..."
bin/install_rust.sh

echo "# Install hadolint..."
wget --quiet --output-document /usr/local/bin/hadolint "https://github.com/hadolint/hadolint/releases/download/v2.14.0/hadolint-Linux-x86_64"
chmod +x /usr/local/bin/hadolint
