#!/bin/bash
set -eu

# Check that Rust source files contains Orange Copyright
find packages -name "*.rs" | while read -r rust_file; do
    if ! grep -q "Copyright (C) 2022 Orange" "$rust_file"; then
        echo "Missing copyright in $rust_file"
	exit 1
    fi
done


