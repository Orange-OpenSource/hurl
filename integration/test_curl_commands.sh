#!/bin/bash
set -eu
find ./tests_ok ./tests_failed -maxdepth 1 -type f -name '*.curl' ! -name '*windows*'|sort | while read -r f; do
    echo "** $f"
    grep -v '^$' <"$f" | grep -v '^#' | while read -r line;  do
        echo "$line"
        cmd="$line --no-progress-meter --output /dev/null --fail"
        echo "$cmd" | bash  || (echo ">>> Error <<<<" && exit 1)
    done
    echo
done

echo "all curl commands have been run with success!"
