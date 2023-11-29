#!/bin/bash
# shellcheck disable=SC1091
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
    echo "  --github-project-path <github full project path> #mandatory"
    echo "      specify github project path"
    echo "        : example: --github-project-path Orange-OpenSource/hurl"
    echo
    echo "  --github-token <github token access> #mandatory"
    echo "      specify github user token with acces to PR api"
    echo "        : example: --github-token ghp_kJvDuaalZidk3nB1uYtgsqMrkQ5Hkh76jh2o"      
    echo
    echo "  --max-days-of-inactivity <days> #optional"
    echo "      maximium days of inactivity before closing a PR"
    echo "        : default value: 30"
    echo "        : example for a month: 7"
    echo
}

function consume-args(){
    dry=false
    github_project_path=
    github_token=
    max_days_of_inactivity=30
    while [[ $# -gt 0 ]] ; do
        case "$1" in
        --help)
            usage
            return 0
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
        --github-project-path)
            if [[ -n ${2:-} ]] ; then
                github_project_path="$2"
                shift
                shift
            else
                print-error "option $1" "can not be null"
                usage >&2
                return 1
            fi
            ;;
        --github-token)
            if [[ -n ${2:-} ]] ; then
                github_token="$2"
                shift
                shift
            else
                print-error "option $1" "can not be null"
                usage >&2
                return 1
            fi
            ;;
        --max-days-of-inactivity)
            if [[ -n ${2:-} ]] ; then
            max_days_of_inactivity="$2"
            shift
            shift
            else
                print-error "option $1" "can not be null"
                usage >&2
                return1
            fi
            ;;
        *)
            print-error "option $1" "is unknown"
            usage >&2
            return 1
            ;;
        esac
    done
    for mandatory_option in github_project_path github_token ; do
        if [[ -z ${!mandatory_option} ]] ; then
            print-error "option --${mandatory_option//_/-}" "is mandatory"
            usage >&2
            return 1
        fi
    done
    if ! (command -v gh >/dev/null 2>&1) ; then
        print-error "packages prerequisites" "github client (gh) has to be installed on your system (https://github.com/cli/cli)"
        usage >&2
        return 1
    fi
}

function is-timestamp-young(){
    timestamp=$1
    max_seconds_of_inactivity=$((max_days_of_inactivity * 24 * 60 * 60))
    actual_timestamp=$(date +%s)
    timestamp_diff=$((actual_timestamp-timestamp))
    if [[ ${timestamp_diff} -le ${max_seconds_of_inactivity} ]] ; then
        return 0
    else
        return 1
    fi
}


# main
script_dir=$(dirname "$0")
source "${script_dir}/github.functions.sh"
source "${script_dir}/common.functions.sh"
init-terminal-colors
consume-args "$@"
github-connect "${github_token}"
github-test-repo "${github_project_path}"
pr_list=$(github-get-pr-number-list "${github_project_path}")
if [[ -z "${pr_list}" ]] ; then
    echo "> There is no opened PR for ${github_project_path}"
else
    while read -r pr_number ; do
        echo "> working on PR ${pr_number} from ${github_project_path}"
        timestamp=$(github-get-pr-last-update-timestamp "${github_project_path}" "${pr_number}")
        if is-timestamp-young "${timestamp}" ; then
            comment="✅ This PR remains open because it is younger than ${max_days_of_inactivity} days"
        else
            comment="📆 This PR has been closed because there is no activity (commits/comments) for more than ${max_days_of_inactivity} days 😥. Feel free to reopen it with new commits/comments."
            if [[ ${dry} == false ]] ; then
                if ! result=$(github-close-pr "${github_project_path}" "${pr_number}" "${comment}" 2>&1) ; then
                    print-error "${FUNCNAME[0]}" "$(head -1 <<< "${result}")"
                    return 1
                fi
            fi
        fi
        echo -e "  - ${comment}"
    done < <(echo "${pr_list}")
fi

