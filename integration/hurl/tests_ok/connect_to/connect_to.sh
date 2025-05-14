#!/bin/bash
set -Eeuo pipefail

hurl --connect-to foo.com:80:localhost:8000 --connect-to bar.com:80:localhost:8000 --connect-to baz.com:80:localhost:8000 --verbose tests_ok/connect_to/connect_to.hurl
