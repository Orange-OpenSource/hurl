#!/bin/bash
set -Eeuo pipefail
hurl tests_ok/multipart_form_data.hurl --verbose
