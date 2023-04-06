#!/bin/bash
set -Eeuo pipefail
hurl tests_error_parser/json_unexpected_character.hurl
