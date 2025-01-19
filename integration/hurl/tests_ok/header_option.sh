#!/bin/bash
set -Eeuo pipefail

hurl tests_ok/header_option.hurl --header 'key: from-cli' --variable my_header='key: from-variable' --variable my_key=key-from-variable --variable my_value=value-from-variable
