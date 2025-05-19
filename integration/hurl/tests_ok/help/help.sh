#!/bin/bash
set -Eeuo pipefail

# In CI, --help is wrapped on a 100 columns wide terminal.
hurl --help
