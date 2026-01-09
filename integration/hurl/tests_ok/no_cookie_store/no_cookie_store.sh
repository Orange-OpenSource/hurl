#!/bin/bash
set -Eeuo pipefail

hurl --no-cookie-store tests_ok/no_cookie_store/no_cookie_store.hurl
