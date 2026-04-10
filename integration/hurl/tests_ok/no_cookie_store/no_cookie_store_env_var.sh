#!/bin/bash
set -Eeuo pipefail

export HURL_NO_COOKIE_STORE=1
hurl tests_ok/no_cookie_store/no_cookie_store.hurl
