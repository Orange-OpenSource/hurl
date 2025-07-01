#!/bin/bash
set -Eeuo pipefail

# functions
function init_terminal_colors(){
    color_red=$(echo -ne "\033[1;31m")
    color_green=$(echo -ne "\033[1;32m")
    color_yellow=$(echo -ne "\033[1;33m")
    color_reset=$(echo -ne "\033[0m")
}

function usage(){
    echo
    echo "Usage: $(basename "$0") [Options]... file1 file2..."
    echo
    echo "Options: #mandatory #optional"
    echo
    echo "  --help #optional"
    echo "      This help text"
    echo
    echo "  --github-token <github token access> #mandatory"
    echo "      specify github user token"
    echo "        : example: --github-token ghp_kJvDuaalZidk3nB1uYtgsqMrkQ5Hkh76jh2o"
}

function consume_args(){
    github_token=
    files_count=0
    while [[ $# -gt 0 ]] ; do
        case "$1" in
        --help)
            usage
            exit 0
            ;;
        --github-token)
            if [[ -n ${2:-} ]] ; then
                github_token="$2"
                shift
                shift
            else
                echo "${color_red}Error${color_reset} - Option $1 can not be null."
                usage >&2
                return 1
            fi
            ;;
        *)
            if [[ -f ${1} ]] ; then
                files+=("$1")
                files_count=$((files_count+1))
                shift
            else
                echo "${color_red}Error${color_reset} - $1 is not a file or is not readable"
                usage >&2
                return 1
            fi
            ;;
        esac
    done
    if [[ -z "${github_token}" ]] ; then
        echo "${color_red}Error${color_reset} - Option --github_token is mandatory."
        usage >&2
        return 1
    fi
    if [[ $files_count == 0 ]] ; then
        echo "${color_red}Error${color_reset} - You must provide at least one file for analysis."
        usage >&2
        return 1
    fi
    if ! (command -v zizmor >/dev/null 2>&1) ; then
        echo "${color_red}Error${color_reset} - Zizmor has to be installed on your system (https://woodruffw.github.io/zizmor/installation)."
        return 1
    fi
}

function set_zizmor_conf(){
    cat <<- "EOF" > "$1"
  rules:
    excessive-permissions:
      ignore:
        - accept-pull-request.yml
        - auto-close-inactive-pr.yml
        - coverage.yml
        - extra-package.yml
        - release.yml
        - update-actions.yml
        - update-branch-version.yml
        - update-crates.yml
    unpinned-uses:
      config:
        policies:
          "*": ref-pin
EOF
}

# main
init_terminal_colors
consume_args "$@"
zizmor --version
conf="/tmp/conf"
set_zizmor_conf "${conf}"
error_count=0

for file in "${files[@]}" ; do
    # disable release.yml for now because output vars have to be rewrited from scratch"
    if [[ "${file}" =~ release.yml ]] ; then
        echo "${color_yellow}$file is disabled for now because output vars have to be rewrited from scratch${color_reset}"
        continue
    fi
    # disable accept-pull-request.yml for now because input vars have to be rewrited from scratch"
    if [[ "${file}" =~ accept-pull-request.yml ]] ; then
        echo "${color_yellow}$file is disabled for now because input vars have to be rewrited from scratch${color_reset}"
        continue
    fi
    tmpfile="/tmp/$(basename "${file}")"
    (sed "s/❌//g" "${file}" 2>/dev/null || true) | \
        (sed "s/✅//g" "${file}" 2>/dev/null || true) | \
            (sed "s/🔨//g" "${file}" 2>/dev/null || true) | \
                (sed "s/🕗//g" "${file}" 2>/dev/null || true) > "${tmpfile}"
    echo "> ${file} (tmp file: ${tmpfile}):"
    zizmor --no-progress --config "${conf}" --gh-token "${github_token}" "${tmpfile}" || error_count=$((error_count+1))
done
if [[ $error_count -gt 0 ]] ; then
    echo "${color_red}There are problems with github workflows${color_reset}"
    exit 1
else
    echo "${color_green}No problem with github workflows${color_reset}"
fi
