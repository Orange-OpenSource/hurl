#!/bin/bash
set -Eeuo pipefail

ruff --version
ruff format --check
ruff check

