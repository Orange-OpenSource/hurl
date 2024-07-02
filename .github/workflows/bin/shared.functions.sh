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

function log_warning(){
    title=$1
    message=$2
    log "${color_yellow}" "${title}" "${message}"
}

function log_success(){
    title=$1
    message=$2
    log "${color_green}" "${title}" "${message}"
}

function log_running(){
    title=$1
    message=$2
    log "${color_cyan}" "${title}" "${message}"
}

function github_connect(){
    if [[ -n ${GITHUB_TOKEN:-} ]] ; then
        connect=$(gh auth login 2>&1 || true)
        if ! result=$(gh auth status 2>&1) ; then
            log_error "${FUNCNAME[0]}" "$(head -3 <<< "${connect}")"
            log_error "${FUNCNAME[0]}" "$(head -3 <<< "${result}")"
            return 1
        else
            log_warning "${FUNCNAME[0]}" "As \$GITHUB_TOKEN is set, it has been used for github auth"
            return 0
        fi
    else
        if [[ $# -ne 1 ]] ; then
            log_error "internal function ${FUNCNAME[0]}" "please provide one parameter, ${FUNCNAME[0]} \$github_token"
            return 1
        else
            token=$1
            connect=$(gh auth login --with-token 2>&1 <<< "${token}" || true)
            if ! result=$(gh auth status 2>&1) ; then
                log_error "${FUNCNAME[0]}" "$(head -1 <<< "${connect}")"
                log_error "${FUNCNAME[0]}" "$(head -1 <<< "${result}")"
                return 1
            else
                return 0
            fi
        fi
    fi
}

function github_test_repo(){
    if [[ $# -ne 1 ]] ; then
        log_error "internal function ${FUNCNAME[0]}" "please provide one parameter, ${FUNCNAME[0]} \$github_project_path"
        return 1
    else
        project_path=$1
        if ! result=$(gh repo view "${project_path}" 2>&1) ; then
            log_error "${FUNCNAME[0]}" "$(head -1 <<< "${result}")"
            return 1
        else
            return 0
        fi
    fi
}

function github_get_latest_release(){
    if [[ $# -ne 1 ]] ; then
        log_error "internal function ${FUNCNAME[0]}" "please provide one parameter, ${FUNCNAME[0]} \$github_project_path"
        return 1
    else
        project_path=$1
        if ! result=$(gh release list --repo "${project_path}" 2>&1) ; then
            log_error "${FUNCNAME[0]}" "$(head -1 <<< "${result}")"
            return 1
        else
            latest=$(echo "$result" | grep Latest | cut --field 3)
            if [[ -z $latest ]] ; then
                log_error "Latest version" "Action $project_path does not have any release tagged as latest, please check this repo"
                exit 1
            else
                echo "$latest"
                return 0
            fi
        fi
    fi
}

function github_get_release_changelog(){
    if [[ $# -ne 2 ]] ; then
        log_error "internal function ${FUNCNAME[0]}" "please provide one parameter, ${FUNCNAME[0]} \${project_path} \$tag"
        return 1
    else
        project_path=$1
        tag=$2
        if ! result=$(gh release view --json body --repo "${project_path}" "${tag}" 2>&1) ; then
            log_error "${FUNCNAME[0]}" "$(head -1 <<< "${result}")"
            return 1
        else
            echo "${result}" | jq -rc .body
        fi
    fi
}

function github_get_pr_number_list(){
    if [[ $# -ne 1 ]] ; then
        log_error "internal function ${FUNCNAME[0]}" "please provide one parameter, ${FUNCNAME[0]} \$github_project_path"
        return 1
    else
        project_path=$1
        if result=$(gh pr list --state open --json number --jq '.[]|.number' --repo "${project_path}" 2>&1) ; then
            sort -V <<< "${result}"
            return 0
        else
            log_error "${FUNCNAME[0]}" "$(head -1 <<< "${result}")"
            return 1
        fi
    fi
}

function github_get_pr_last_update_timestamp(){
    if [[ $# -ne 2 ]] ; then
        log_error "internal function ${FUNCNAME[0]}" "please provide one parameter, ${FUNCNAME[0]} \$github_project_path \$pr_number"
        return 1
    else
        project_path=$1
        number=$2
        if result=$(gh pr view --json updatedAt --jq .updatedAt --jq '.updatedAt|fromdate|tostring' "https://github.com/${project_path}/pull/${number}" 2>&1) ; then
            echo "${result}"
            return 0
        else
            log_error "${FUNCNAME[0]}" "$(head -1 <<< "${result}")"
            return 1
        fi
    fi
}

function github_comment_pr(){
    if [[ $# -ne 3 ]] ; then
        log_error "internal function ${FUNCNAME[0]}" "please provide one parameter, ${FUNCNAME[0]} \$github_project_path \$pr_number \$pr_comment"
        return 1
    else
        project_path=$1
        pr_number=$2
        comment=$3
        if ! result=$(gh pr comment "${pr_number}" --repo "${project_path}" --body "${comment}") ; then
            log_error "${FUNCNAME[0]}" "$(head -1 <<< "${result}")"
            return 1
        else
            return 0
        fi
    fi
}

function github_close_pr(){
    if [[ $# -ne 3 ]] ; then
        log_error "internal function ${FUNCNAME[0]}" "please provide one parameter, ${FUNCNAME[0]} \$github_project_path \$pr_number \$pr_comment"
        return 1
    else
        project_path=$1
        pr_number=$2
        comment=$3
        if ! result=$(gh pr close "${pr_number}" --repo "${project_path}" --comment "${comment}") ; then
            log_error "${FUNCNAME[0]}" "$(head -1 <<< "${result}")"
            return 1
        else
            return 0
        fi
    fi
}

# main
init_terminal_colors
