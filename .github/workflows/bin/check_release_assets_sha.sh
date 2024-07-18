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
    echo "  --release <[0-9].[0-9].[0.9]> #mandatory"
    echo "      specify the github release version, in semver format"
    echo "        : example: --release 4.3.0"
    echo
    echo "  --github-repo <owner/repo> #mandatory"
    echo "      specify github repo"
    echo "        : example: orange-opensource/hurl"
    echo
    echo "  --github-token <github token access> #mandatory"
    echo "      specify github user token with access to PR api"
    echo "        : example: --github-token ghp_kJvDuaalZtyhinB1uYtgsqMrkQ5Hkh76jh2o"      
    echo
}

function consume_args(){
    release=""
    github_repo=""
    github_token=""
    while [[ $# -gt 0 ]] ; do
        case "$1" in
        --help)
            usage
            exit 0
            ;;
        --release)
            if [[ -n ${2:-} ]] ; then
                release="$2"
                shift
                shift
            else
                log_error "option $1" "can not be null"
                usage >&2
                return 1
            fi
            ;;
        --github-repo)
            if [[ -n ${2:-} ]] ; then
                github_repo="$2"
                shift
                shift
            else
                log_error "option $1" "can not be null"
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
    if [[ -z $release ]] ; then
        log_error "option --release" "is mandatory"
        usage >&2
        return 1
    elif [[ ! $release =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] ; then
        log_error "option --release" "does not match semver format. Have to be x.y.z, for example 4.3.0"
        usage >&2
        return 1
    elif [[ -z $github_repo ]] ; then
        log_error "option --github_repo" "is mandatory"
        usage >&2
        return 1
    elif [[ -z $github_token ]] ; then
        log_error "option --github_token" "is mandatory"
        usage >&2
        return 1
    elif ! (command -v gh >/dev/null 2>&1) ; then
        log_error "packages prerequisites" "github client (gh) has to be installed on your system (https://github.com/cli/cli)"
        usage >&2
        return 1
    elif ! (command -v jq >/dev/null 2>&1) ; then
        log_error "packages prerequisites" "jq has to be installed on your system"
        usage >&2
        return 1
    elif ! (command -v sha256sum >/dev/null 2>&1) ; then
    ¦   log_error "packages prerequisites" "jq has to be installed on your system"
    ¦   usage >&2
    ¦   return 1
    fi
}

function prerequisites(){
    script_dir=$(dirname "$0")
    source "${script_dir}/shared.functions.sh"
    init_terminal_colors
    declare color_red
    declare color_green
    declare color_reset
}

function is_release(){
    release="$1"
    gh release view "${release}" --json name --jq ".name" >/dev/null 2>&1 && gh_exit_code=0 || gh_exit_code=$?
    if [[ $gh_exit_code -eq 0 ]] ; then
        echo "${release} is a release in ${github_repo}"
    else
        log_error "release information" "${release} release does not exist in ${github_repo}"
        return 1
    fi
}

function download_assets(){
    release="$1"
    tmpdir="$2"
    gh release download "${release}" --clobber --dir "${tmpdir}" 2>&1 && gh_exit_code=0 || gh_exit_code=$?
    if [[ $gh_exit_code -eq 0 ]] ; then
        echo "Assets downloaded for release ${release} from repo ${github_repo}"
    else
        log_error "download assets" "error during assets download for release ${release} from repo ${github_repo}"
        clean_tmpdir "${tmpdir}"
        return 1
    fi

}

function extract_assets_sha(){
    inputdir="$1"
    files=$(ls "${inputdir}" --ignore=*.sha256* 2>/dev/null || true)
    for file in ${files} ; do
       sha=$(sha256sum "${inputdir}/${file}" | cut --delimiter " " --field 1 | tr -d ' ')
       echo "$sha" > "${inputdir}/${file}.sha256.new"
    done
}

function compare_assets_sha(){
    inputdir="$1"
    files=$(ls "${inputdir}/"*.sha256 || true)
    if [[ -z $files ]] ; then
        log_error "sha256 files" "there is no *.sha256 files for ${release} from repo ${github_repo}"
        clean_tmpdir "${tmpdir}"
        return 1
    fi
    error_count=0
    for file in ${files} ; do
        sdiff "${file}" "${file}.new" >/dev/null 2>&1 && sdiff_exit_code=0 || sdiff_exit_code=$?
        if [[ $sdiff_exit_code -eq 0 ]] ; then
            echo "$(basename --suffix=.sha256 "${file}") - ${color_green}OK${color_reset}"
        else
            echo "$(basename --suffix=.sha256 "${file}") - ${color_red}sha256 mismatch with published asset in release ${release}${color_reset}"
            echo "  published sha256 : $(head -1 "${file}")"
            echo "  actual sha256    : ${color_red}$(head -1 "${file}".new)${color_reset}"
            error_count=$((error_count+1))
        fi
    done
    if [[ $error_count -gt 0 ]] ; then
        log_error "sha256 mismatch" "Please verify assets integrity for release ${release} from repo ${github_repo}"
        clean_tmpdir "${tmpdir}"
        return 1
    fi
}

function clean_tmpdir(){
    tmpdir="$1"
    if [[ -d "${tmpdir}" ]] ; then
        rm -fr "${tmpdir}"
    fi
}

# main
prerequisites
consume_args "$@"
is_release "${release}"
tmpdir="/tmp/assets/${release}/${RANDOM}"
mkdir -p "${tmpdir}"
download_assets "${release}" "${tmpdir}"
extract_assets_sha "${tmpdir}"
compare_assets_sha "${tmpdir}"

