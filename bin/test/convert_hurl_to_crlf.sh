#!/bin/bash
set -Eeuo pipefail

# functions
function init_terminal_colors(){
    color_red=$(echo -ne "\033[1;31m")
    color_green=$(echo -ne "\033[1;32m")
    color_yellow=$(echo -ne "\033[1;33m")
    color_cyan=$(echo -ne "\033[1;36m")
    color_reset=$(echo -ne "\033[0m")
}

function log(){
    color="$1"
    title="$2"
    message="$3"
    if basename "$0" >/dev/null 2>&1 ; then
        parent=$(basename "$0")
    else
        parent="."
    fi
    echo "${color}${parent}: ${title}: ${message}${color_reset}"
}

function log_error(){
    title="$1"
    message="$2"
    log "${color_red}" "${title}" "${message}"
}

function usage(){
    echo
    echo "Usage: $(basename "$0") [Options]..."
    echo
    echo "Options: #mandatory #optional"
    echo
    echo "  --help #optional"
    echo "      This help text"
    echo
    echo "  --dest-dir #optional"
    echo "      path where actual dir is copied to convert hurl files to CRLF"   
    echo "        : default value: /tmp/hurl_tmp"
    echo "        : example: --dest-dir /tmp/my_tmp_dir"
    echo
}

function consume_args(){
    dest_dir=/tmp/hurl_tmp
    while [[ $# -gt 0 ]] ; do
        case "$1" in
        --help)
            usage
            exit 0
            ;;
        --dest-dir)
            dest_dir=$2
            shift
            shift
            ;;
        *)
            log_error "option $1" "is unknown"
            usage >&2
            return 1
            ;;
        esac
    done
}

function prerequisites(){
    if ! command -V unix2dos --version >/dev/null 2>&1 ; then
        log_error "Prerequisites" "Please install dos2unix first."
        exit 1
    fi
}

function is_not_bad_format(){
    file="$1"
    if (file "${file}" | grep "very short file" >/dev/null 2>&1) ; then
        return 1
    else
        return 0
    fi
}

function is_not_assert_body(){
    file="$1"
    if (sed '/^HTTP/{N;s/\n/ /;}' "${file}" | grep -E "^HTTP.*\`\`\`" > /dev/null 2>&1) ; then
       return 1
    else
       return 0
    fi 
}

# main
init_terminal_colors
prerequisites
consume_args "$@"
echo "${color_cyan}# Clone this dir ${PWD} to ${dest_dir}${color_reset}"
echo
if cp -frp . "${dest_dir}" ; then
    echo "  - current dir ${PWD} : ${color_green}copied to ${dest_dir}${color_reset}"
else
    echo "  - current dir ${PWD} : ${color_red}copied to ${dest_dir}${color_reset}"
    exit 1
fi
cp -frp . "${dest_dir}"
echo
echo "${color_cyan}# Unix2dos all *.hurl files in ${dest_dir}${color_reset}"
echo
exit_code=0
while read -r hurl_file ; do
    if is_not_bad_format "${hurl_file}" ; then
        if is_not_assert_body "${hurl_file}" ; then
            if unix2dos "${hurl_file}" ; then
                echo "  - ${hurl_file} : ${color_green}converted to dos${color_reset}"
            else
                echo "  - ${hurl_file} : ${color_red}not converted to dos${color_reset}"
                exit_code=1
            fi
        else
            echo "  - ${hurl_file} : ${color_yellow}not converted to dos because it contains body implicit asserts${color_reset}"
        fi
    else
        echo "  - ${hurl_file} : ${color_yellow}not converted (bad file format)${color_reset}"
    fi
done < <(find "${dest_dir}/integration/hurl" -name "*.hurl")
exit "${exit_code}"
