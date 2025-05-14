#!/bin/bash
set -Eeuo pipefail

rm -f build/cookies.txt
hurl --cookie-jar build/cookies.txt --no-output tests_ok/cookie/cookie_jar.hurl
cat build/cookies.txt
