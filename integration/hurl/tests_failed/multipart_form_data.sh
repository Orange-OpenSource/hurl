#!/bin/bash
set -Eeuo pipefail
hurl tests_failed/multipart_form_data.hurl
