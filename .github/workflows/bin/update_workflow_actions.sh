#!/bin/bash
#set -Eeuo pipefail

# functions

function usage(){
    echo
    echo "Usage: $(basename "$0") [Options]..."
    echo
    echo "Options: #mandatory #optional"
    echo
    echo "  --help #optional"
    echo "      This help text"
    echo
    echo "  --dry #optional"
    echo "      simulate execution"   
    echo "        : default value: false"
    echo "        : example: --dry"
    echo
    echo "  --github-token <github token access> #mandatory"
    echo "      specify github user token with acces to PR api"
    echo "        : example: --github-token ghp_kJvDuaalZtyhinB1uYtgsqMrkQ5Hkh76jh2o"      
    echo
}

function consume_args(){
    dry=false
    github_token=""
    while [[ $# -gt 0 ]] ; do
        case "$1" in
        --help)
            usage
            exit 0
            ;;
        --dry)
            if [[ ${2:-} =~ true|false ]] ; then
                dry=$2
                shift
                shift
            else
                dry=true
                shift
            fi
            ;;
        --github-token)
            if [[ -n ${2:-} ]] ; then
                github_token="$2"
                shift
                shift
            else
                log_error "option $1" "can not be null"
                usage >&2
                return 1
            fi
            ;;
        *)
            log_error "option $1" "is unknown"
            usage >&2
            return 1
            ;;
        esac
    done
    if [[ -z $github_token ]] ; then
        log_error "option --github_token" "is mandatory"

        usage >&2
        return 1
    fi
    if ! (command -v gh >/dev/null 2>&1) ; then
        log_error "packages prerequisites" "github client (gh) has to be installed on your system (https://github.com/cli/cli)"
        usage >&2
        return 1
    fi
}

function get_uses_action_list(){
    grep --recursive --no-filename "uses: " "$script_dir"/../* | \
        grep @ | \
            grep --invert-match "#" | \
                cut --delimiter ':' --field 2- | \
                    tr -d ' \r\t' | \
                        sort -u | \
                            tr '@' ' '
}

# main
script_dir=$(dirname "$0")
source "${script_dir}/shared.functions.sh"
init_terminal_colors
consume_args "$@"

echo -e "\n--------------------------------------------------------"
echo -e "### Check actions\n\n"
while read -r action version; do
    update_files=$(grep --recursive --files-with-matches "$action@$version" "$script_dir"/../* | xargs realpath)
    latest=$(github_get_latest_release "$action")
    if [[ "$version" == "$latest" ]] ; then
        echo -e "\n- $action@$version ${color_green}newest${color_reset}"
    else
        if [ "$dry" == "true" ] ; then
            echo -e "\n- $action@$version ${color_red}please update to max stable version ${latest}${color_reset}"
            echo "$update_files" | sed "s/^/  - /g"
        else
            while read file ; do
                sed -i "s#$action@$version#$action@$latest#g" "$file" || true
                if grep -E "$action@$version$|$action@$version " "$file" ; then
                    echo -e "\n- $action@$version ${color_red} fails to update to ${latest}${color_reset}"
                    echo "  - ${color_red} please check write permissions on $file" 
                    exit 1
                else
                    echo -e "\n- $action@$version ${color_green}updated to ${latest}${color_reset}"
                    echo "$update_files" | sed "s/^/  - /g"
                fi
            done < <(echo "$update_files")
        fi

    fi
done < <(get_uses_action_list)

