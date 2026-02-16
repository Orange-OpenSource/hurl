#!/bin/bash
set -Eeuo pipefail

# We check the help without any color
export NO_COLOR=1

# In pty tests, --help is wrapped on a 100 columns wide terminal.
hurl --help
