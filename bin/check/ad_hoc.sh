#!/bin/bash
set -Eeuo pipefail

color_red=$(echo -e "\033[1;31m")
color_reset=$(echo -e "\033[0m")
errors_count=0

# Check that Rust source files contains Orange Copyright
find packages -name "*.rs" | while read -r rust_file; do
    if ! grep -q "Copyright (C) 2022 Orange" "$rust_file"; then
        echo "Missing copyright in $rust_file"
	exit 1
    fi
done


