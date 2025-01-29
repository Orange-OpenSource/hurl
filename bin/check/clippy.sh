#!/bin/bash
set -Eeuo pipefail

cargo clippy --all-targets
