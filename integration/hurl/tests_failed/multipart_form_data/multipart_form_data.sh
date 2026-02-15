#!/bin/bash
set -Eeuo pipefail

hurl --no-color tests_failed/multipart_form_data/multipart_form_data.hurl
