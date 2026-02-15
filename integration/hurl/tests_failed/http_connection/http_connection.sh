#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_failed/http_connection/http_connection.hurl
