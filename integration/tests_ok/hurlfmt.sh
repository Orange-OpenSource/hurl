#!/bin/bash
set -Eeuo pipefail
echo 'GET http://localhost:8000/hello' | hurlfmt --color

