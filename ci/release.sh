#!/bin/bash
set -e

cargo build --release --verbose
strip target/release/hurl
strip target/release/hurlfmt

target/release/hurl --version

