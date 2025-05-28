#!/bin/bash
set -Eeuo pipefail

hurl tests_ok/querystring/querystring_params.hurl
