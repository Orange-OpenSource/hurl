#!/bin/bash
set -e
cargo install cargo-deb
cargo deb
cp target/debian/*.deb target/upload

