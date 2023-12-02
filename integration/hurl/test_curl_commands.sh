#!/bin/bash
set -Eeuo pipefail

while read -r test_file ; do
    echo "** ${test_file}"
    while read -r line;  do
        echo "${line}"
        cmd="${line} --no-progress-meter --output /dev/null --fail"
        echo "${cmd}" | bash  || (echo ">>> Error <<<<" && exit 1)
    done < <( (grep -v '^$' "${test_file}" || true) | (grep -v '^#' || true) )
    echo
done < <( find ./tests_ok ./tests_failed -maxdepth 1 -type f -name '*.curl' ! -name '*windows*' | sort )

echo "all curl commands have been run with success!"

