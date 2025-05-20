#!/bin/bash
set -Eeuo pipefail

hurl --insecure --verbose tests_ssl/insecure.hurl
