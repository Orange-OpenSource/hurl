#!/bin/bash
set -Eeuo pipefail

rm -f build/no_cookie_store_output.txt

hurl --verbose --no-cookie-store --cookie-jar build/no_cookie_store_output.txt tests_ok/no_cookie_store/no_cookie_store.hurl
cat build/no_cookie_store_output.txt