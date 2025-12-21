#!/bin/bash
set -Eeuo pipefail

hurl --digest -u "username:password" tests_ok/digest/digest.hurl
