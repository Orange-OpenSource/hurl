#!/bin/bash
set -Eeuo pipefail

function init_colors(){
    color_red=$(echo -ne "\033[1;31m")
    color_green=$(echo -ne "\033[1;32m")
    color_yellow=$(echo -ne "\033[1;33m")
    color_cyan=$(echo -ne "\033[1;36m")
    color_reset=$(echo -ne "\033[0m")
}

function prerequisites(){
if ! (command -v shellcheck >/dev/null 2>&1) ; then
    echo "${color_red}Error${color_reset} - Shellcheck has to be installed on your system (https://github.com/koalaman/shellcheck?tab=readme-ov-file#installing)."
    exit 1
fi
if ! (command -v yq >/dev/null 2>&1) ; then
    echo "${color_red}Error${color_reset} - Yq has to be installed on your system (https://github.com/mikefarah/yq#install)."
    exit 1
fi
}

function shellcheck_files(){
    echo "# shellcheck sh scripts files"
    find . -type f -name '*.sh' -print0  | xargs -0 shellcheck
    echo "${color_green}No problem with sh script files${color_reset}"
    echo
}

function shellcheck_gitflow(){
    echo "# shellcheck sh scripts inside Github Workflow"
    error_count=0
    for yaml in .github/workflows/*yml ; do
        echo "${color_cyan}  ======================================================================="
        echo "${color_cyan}  >   shellchecking ${yaml}..."
        echo "${color_cyan}  ======================================================================="
        echo "${color_cyan}  |${color_reset}"
        file=$(basename "${yaml}")
	if [ "${file}" == "accept-pull-request.yml" ] || [ "${file}" == "release.yml" ] ; then
            echo "${color_cyan}  |${color_yellow} Disabled for now because output vars have to be rewrited from scratch"
	    echo "${color_cyan}  |${color_reset}"
            continue
        fi
        tmp="/tmp/${file}"
	yq '.jobs[] | select(.["runs-on"] | test("windows") | not).steps[] | select(.run != null) | .run' "${yaml}" \
	    | sed -E 's/\$\{\{[[:space:]]*/\${/g' \
            | sed -E 's/[[:space:]]*\}\}/}/g' \
            | sed -E 's/(\$\{[^}]*)\.([^}]*)/\1-\2/g' \
            | sed -E 's/(\$\{[^}]*)\.([^}]*)/\1-\2/g' \
            | sed -E 's/(\$\{[^}]*)\.([^}]*)/\1-\2/g' \
            | sed -E 's/(\$\{[^}]*)\.([^}]*)/\1-\2/g' \
	    > "${tmp}"
        if ! output=$(shellcheck --exclude=SC2148 --color=always "${tmp}" 2>&1) ; then
            echo "${output}" | sed "/In.*${file}.*/d" | sed "s/^/${color_cyan}  |${color_reset}  /g"
            error_count=$((error_count+1))
	else
	    echo "${color_cyan}  | ${color_green}No problem with ${yaml}${color_reset}"
	    echo "${color_cyan}  |"
        fi
        rm "${tmp}"
        echo
    done
    if [[ $error_count -gt 0 ]] ; then
        echo "${color_red}There are shellcheck errors with sh scripts inside github workflows${color_reset}"
        exit 1
    else
        echo "${color_green}No problem with sh scripts inside github workflows${color_reset}"
    fi
}

# main
init_colors
prerequisites
shellcheck_files
shellcheck_gitflow

