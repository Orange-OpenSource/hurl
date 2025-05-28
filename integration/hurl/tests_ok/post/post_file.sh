#!/bin/bash
set -Eeuo pipefail

hurl --variable filename=data.bin tests_ok/post/post_file.hurl
