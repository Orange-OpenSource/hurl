#!/bin/bash
set -Eeuo pipefail

# init colors
color_grey=$(echo -ne "\033[0;30m")
color_red=$(echo -ne "\033[1;31m")
color_green=$(echo -ne "\033[1;32m")
color_yellow=$(echo -ne "\033[1;33m")
color_blue=$(echo -ne "\033[1;34m")
color_reset=$(echo -ne "\033[0m")

# functions
check_args() {
    # init args
    run_mode="$1"

    # check args
    if [ -n "${run_mode}" ] && [ "${run_mode}" != "--check" ]; then
        echo "${color_yellow}"
        echo "Usage:"
        echo "  $(basename "$0") # update all crates to latest version into Cargo.toml and Cargo.lock files in actual dir/subdir"
        echo "  $(basename "$0") --check # display upgradable crates from Cargo.toml and Cargo.lock files in actual dir/subdir"
        echo "${color_reset}"
        return 1
    fi
}

convert_toml_crates_to_key_value() {
    # init args
    toml_file="$1"

    # convert toml crates to key value
    sed -n "/dependencies\]/,/^$/p" "${toml_file}" |
        grep --extended-regexp --invert-match "^\[|path|^$|^#" |
            cut --delimiter ',' --field 1 |
                sed "s/version//g" |
                    sed "s/=/ = /g" |
                        tr -d '"{=}' |
                            tr -d "'" |
                                tr -s ' ' |
                                    sort
}

get_crate_latest_version() {
    # init args
    crate_url="$1"

    # get crate latest version
    crate_object=$(curl -kLs "${crate_url}" || true)
    crate_max_stable_version=$(echo "${crate_object}" | (jq -r .crate.max_stable_version 2>/dev/null || true))
    last_version=$(echo "${crate_max_stable_version}" | (grep --extended-regexp --only-matching "^[0-9]*\.[0-9]*\.[0-9]*" || true))
    echo "${last_version}"
}

get_crate_github_release_body(){
    # init args
    crate_url="$1"
    crate_release="$2"

    # get crate github repository
    crate_object=$(curl -kLs "${crate_url}" || true)
    crate_repository=$(echo "${crate_object}" | (jq -r .crate.repository 2>/dev/null || true))
    github_repository=$(echo "${crate_repository}" | (grep --extended-regexp "^https://github.com" || true))

    # get github release body
    if [ -n "${github_repository}" ] ; then
        owner_repo=$(echo "${github_repository}" | cut --delimiter "/" --field 4-)
        tag_name=$(curl -kLs --header "Accept: application/vnd.github+json" https://api.github.com/repos/"${owner_repo}"/git/refs/tags | (jq -r .[].ref 2>/dev/null || true) | (grep "${crate_release}$" || true) | cut --delimiter "/" --field 3)
        release_body=$( (curl -kLs --header "Accept: application/vnd.github+json" https://api.github.com/repos/"${owner_repo}"/releases/tags/"${tag_name}" || true) | (jq -r .body 2>/dev/null || 
true) )
    else
        release_body="null"
    fi

    # display release infos
    echo -e "\n        ${color_grey}${crate_repository}${color_reset}\n"
    if [ -n "${release_body}" ] && [ "${release_body}" != "null" ] ; then
        echo -n "${color_grey}"
        echo "${release_body}" | tr -s ' ' | sed 's/```//g' | sed "s/^-//g" | sed "s/^/        /g"
        echo -e "${color_reset}\n"
    fi
}

update_crate_version_in_toml() {
    # init args
    crate="$1"
    actual_version="$2"
    last_version="$3"
    toml_file="$4"

    # update crate version in toml
    sed -i -- "s/^${crate}.*=.*{.*version.*=.*\"${actual_version}\"/${crate} = { version = \"${last_version}\"/g" "${toml_file}"
    sed -i -- "s/^${crate}.*=.*\"${actual_version}\"/${crate} = \"${last_version}\"/g" "${toml_file}"
    if grep --extended-regexp --silent "${crate}.*=.*${last_version}" "${toml_file}"; then
        echo "${color_blue}updated to ${last_version}${color_reset}"
    else
        echo "${color_red}error, update to ${last_version} fails, please check ${toml_file} format and syntax, then re-run this script${color_reset}"
        return 1
    fi
}

main() {
    # init vars
    crates_api_root_url="https://crates.io/api/v1/crates"
    arg="${1:-}"
    check_args "${arg}"
    updated_count=0

    # update toml
    for package in packages/*; do
        toml_file="${package}/Cargo.toml"
        echo -e "\n--------------------------------------------------------"
        echo -e "### Crates updates for *${toml_file}*\n"
        while read -r crate actual_version; do
            crate_url="${crates_api_root_url}/${crate}"
            last_version=$(get_crate_latest_version "${crate_url}")
            if [ -z "${last_version}" ]; then
                echo "${color_red}runtime error${color_reset}, i could not get last version from ${crate_url}"
                return 1
            fi
            echo -n "- ${crate} ${actual_version} "
            if [ "${last_version}" == "${actual_version}" ]; then
                echo "${color_green}newest${color_reset}"
            else
                if [ "${arg}" = "--check" ]; then
                    echo "${color_red}please update to max stable version ${last_version}${color_reset}"
                    updated_count=$((updated_count + 1))
                else
                    update_crate_version_in_toml "${crate}" "${actual_version}" "${last_version}" "${toml_file}"
                    get_crate_github_release_body "${crate_url}" "${last_version}"
                fi
            fi
        done < <(convert_toml_crates_to_key_value "${toml_file}")
    done

    # update lock
    if [ "${arg}" != "--check" ]; then
        updated_lock_file="/tmp/updated_locks.list"
        echo -e "\n--------------------------------------------------------"
        echo -e "### Crates updates for *Cargo.lock*\n"
        cargo update --color always -vv 2>&1 |
            (grep -Ev "crates.io index|Removing|Unchanged|cargo tree|^#" || true) |
                tr -s ' ' |
                    sed -r "s/\x1B\[([0-9]{1,3}(;[0-9]{1,2})?)?[mGK]//g" |
                        sed "s/ Updating //g" |
                            sed "s/->//g" > "${updated_lock_file}"
        while read -r crate actual_version last_version ; do
            formatted_actual_version=$(echo "${actual_version}" | (grep --only-matching --extended-regexp "[0-9].*.[0-9].*.[0-9]" || true))
            formatted_last_version=$(echo "${last_version}" | (grep --only-matching --extended-regexp "[0-9].*.[0-9].*.[0-9]" || true))
            crate_url="${crates_api_root_url}/${crate}"
            echo "- ${crate} ${formatted_actual_version} ${color_blue}updated to ${formatted_last_version}${color_reset}"
            get_crate_github_release_body "${crate_url}" "${last_version}"

        done < "${updated_lock_file}"
        echo -e "\n"
    fi

    # display help if update is needed with --check run_mode
    if [ "${updated_count}" -gt 0 ]; then
        echo -e "\n${color_yellow}...Consider re-executing $(basename "$0") without --check option to automatically update ${updated_count} old crates${color_reset}\n"
        return "${updated_count}"
    fi
}

# run
main "$@"

