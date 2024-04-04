#!/bin/bash
set -Eeuo pipefail

color_red=$(echo -e "\033[1;31m")
color_yellow=$(echo -ne "\033[1;33m")
color_green=$(echo -ne "\033[1;32m")
color_reset=$(echo -e "\033[0m")

cmd_find_test_files="find ./tests_ok ./tests_failed -maxdepth 1 -type f -name '*.curl' ! -name '*windows*'"
exclude_lines="^#"
echo -e "\n# curl infos"
which curl
curl --version

echo -e "\n# execute curl commands"
if ! curl --version | grep brotli >/dev/null 2>&1 ; then
    exclude_lines="^#|brotli"
    echo "${color_yellow}! Brotli tests excluded because curl does not contain this functionality in this system${color_reset}"
fi

while read -r test_file ; do
    echo "** ${test_file}"
    cmd_find_lines="(grep -v '^$' \"${test_file}\" || true) | (grep -Ev \"$exclude_lines\" || true)"
    while read -r line;  do
        echo "${line}"
        cmd="${line} --no-progress-meter --output /dev/null --fail"
        eval "$cmd" && exit_code=0 || exit_code=1
        if [[ $exit_code -eq 1 ]] ; then
            echo "${color_red}>>> Error executing curl command${color_reset}"
            echo "cmd=$cmd"
            cmd="${cmd} --verbose"
            result=$(bash -c "$cmd" || true)
            echo "$result"
            echo "${color_red}<<<${color_reset}"
            exit 1
        fi
    done < <(eval "$cmd_find_lines")
    echo
done < <(eval "$cmd_find_test_files")

echo "${color_green}all curl commands have been run with success!${color_reset}"
