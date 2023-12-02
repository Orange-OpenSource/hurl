#!/bin/bash
#!/bin/bash
set -Eeuo pipefail
for hurl_file in "$@"; do
    echo "hurlfmt $hurl_file"
    output_file=/tmp/$(basename "$hurl_file")
    hurlfmt "$hurl_file" >"$output_file"
    if ! diff "$hurl_file" "$output_file"; then
        exit 1
    fi
done

