#!/bin/bash
# shellcheck disable=SC1091,SC2001
set -Eeuo pipefail

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
    echo "      specify github user token with access to PR api"
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

function prerequisites(){
    script_dir=$(dirname "$0")
    source "${script_dir}/shared.functions.sh"
    init_terminal_colors
    declare color_red
    declare color_green
    declare color_reset
}

# main
prerequisites
consume_args "$@"

echo -e "\n--------------------------------------------------------"
echo "### Check actions"
while read -r action version; do
    update_files=$(grep --recursive --files-with-matches "$action@$version" "$script_dir"/../* | tr -d ' ' | sort -u | xargs realpath)
    latest=$(github_get_latest_release "$action")
    latest_tag=$(echo "$latest" | cut -c1-)
    if [[ "$version" == "$latest" ]] ; then
        echo -e "\n- $action@$version ${color_green}newest${color_reset}"
        continue
    else
        if [ "$dry" == "true" ] ; then
            echo -e "\n- $action@$version ${color_red}please update to max stable version ${latest}${color_reset}"
        else
            echo -en "\n- $action@$version "
            while read -r file ; do
                sed -i "s#$action@$version#$action@$latest#g" "$file" || true
                if grep -E "$action@$version$|$action@$version " "$file" ; then
                    echo "${color_red} fails to update to ${latest}${color_reset}"
                    echo "  - ${color_red} please check write permissions on $file" 
                    exit 1
                fi
            done < <(echo "$update_files")
            echo "${color_green}updated to ${latest}${color_reset}"
        fi
    fi
    changelog=$(github_get_release_changelog "$action" "$latest_tag" | fold -w 100 | sed "s/^/    /g")
    echo -e '\n    ```'
    echo "$changelog"
    echo '    ```'
done < <(get_uses_action_list)

