#!/bin/bash
set -Eeuo pipefail

hurl --insecure tests_ssl/insecure.hurl
