#!/bin/bash
set -Eeuo pipefail

# functions

function github-connect(){
    if [[ $# -ne 1 ]] ; then
        print-error "internal function ${FUNCNAME[0]}" "please provide one parameter, ${FUNCNAME[0]} \$github_token"
        return 1
    else
        token=$1
        if ! result=$(gh auth login --with-token 2>&1 <<< "${token}") ; then
            print-error "${FUNCNAME[0]}" "$(head -1 <<< "${result}")"
            return 1
        fi
    fi
}

function github-test-repo(){
    if [[ $# -ne 1 ]] ; then
        print-eror "internal function ${FUNCNAME[0]}" "please provide one parameter, ${FUNCNAME[0]} \$github_project_path"
        return 1
    else
        project_path=$1
        if ! result=$(gh repo view "${project_path}" 2>&1) ; then
            print-error "${FUNCNAME[0]}" "$(head -1 <<< "${result}")"
            return 1
        fi
    fi
}

function github-get-pr-number-list(){
    if [[ $# -ne 1 ]] ; then
        print-error "internal function ${FUNCNAME[0]}" "please provide one parameter, ${FUNCNAME[0]} \$github_project_path"
        return 1
    else
        project_path=$1
        if result=$(gh pr list --state open --json number --jq '.[]|.number' --repo "${project_path}" 2>&1) ; then
            sort -V <<< "${result}"
        else
            print-error "${FUNCNAME[0]}" "$(head -1 <<< "${result}")"
            return 1
        fi
    fi
}

function github-get-pr-last-update-timestamp(){
    if [[ $# -ne 2 ]] ; then
        print-error "internal function ${FUNCNAME[0]}" "please provide one parameter, ${FUNCNAME[0]} \$github_project_path \$pr_number"
        return 1
    else
        project_path=$1
        number=$2
        if result=$(gh pr view --json updatedAt --jq .updatedAt --jq '.updatedAt|fromdate|tostring' "https://github.com/${project_path}/pull/${number}" 2>&1) ; then
            echo "${result}"
        else
            print-error "${FUNCNAME[0]}" "$(head -1 <<< "${result}")"
            return 1
        fi
    fi
}

function github-comment-pr(){
    if [[ $# -ne 3 ]] ; then
        print-error "internal function ${FUNCNAME[0]}" "please provide one parameter, ${FUNCNAME[0]} \$github_project_path \$pr_number \$pr_comment"
        return 1
    else
        project_path=$1
        pr_number=$2
        comment=$3
        if ! result=$(gh pr comment "${pr_number}" --repo "${project_path}" --body "${comment}") ; then
            print-error "${FUNCNAME[0]}" "$(head -1 <<< "${result}")"
            return 1
        fi
    fi
}

function github-close-pr(){
    if [[ $# -ne 3 ]] ; then
        print-error "internal function ${FUNCNAME[0]}" "please provide one parameter, ${FUNCNAME[0]} \$github_project_path \$pr_number \$pr_comment"
        return 1
    else
        project_path=$1
        pr_number=$2
        comment=$3
        if ! result=$(gh pr close "${pr_number}" --repo "${project_path}" --comment "${comment}") ; then
            print-error "${FUNCNAME[0]}" "$(head -1 <<< "${result}")"
            return 1
        fi
    fi
}

