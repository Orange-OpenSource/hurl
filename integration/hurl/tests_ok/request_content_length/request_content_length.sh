#!/bin/bash
set -Eeuo pipefail

hurl tests_ok/request_content_length/request_content_length.hurl
