#!/bin/bash
set -Eeuo pipefail
hurl --verbose tests_ok/multipart_form_data.hurl
