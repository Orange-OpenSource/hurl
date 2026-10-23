#!/bin/bash
set -Eeuo pipefail

hurl --verbose tests_failed/curl_injection/curl_injection.hurl

