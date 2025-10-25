#!/bin/bash
set -Eeuo pipefail

echo "Test Caddy Hurl tests <https://github.com/caddyserver/caddy>"
echo "------------------------------------------------------------"

work_dir=build/
mkdir -p build/
cd "$work_dir".
rm -rf caddy || true
git clone --branch hurl-tests --no-depth https://github.com/caddyserver/caddy.git
cd caddy

# TODO: Install caddy depending on the system

caddy stop || true
caddy start 2>/dev/null

hurl --jobs 1 --variables-file caddytest/spec/hurl_vars.properties --test caddytest/spec/

caddy stop
