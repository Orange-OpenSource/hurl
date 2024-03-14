#!/bin/bash
set -Eeuo pipefail
hurl --glob 'does_not_exist/*.hurl'

