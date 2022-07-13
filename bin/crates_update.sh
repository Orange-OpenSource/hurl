#!/usr/bin/env bash
# init exit workflow
set -o errexit
set -o pipefail

# init colors
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
        grep --extended-regexp --invert-match "^\[|path|^$" |
        cut --delimiter ',' --field 1 |
        sed "s/version//g" |
        tr -d '"{=' |
        tr -d "'" |
        tr -s ' ' |
        sort
}

get_crate_latest_version() {
    # init args
    crate_url="$1"

    # get crate latest version
    crate_object=$(curl -kLs "${crate_url}" || true)
    crate_max_stable_version=$(echo "${crate_object}" | (jq -r .crate.max_stable_version || true))
    last_version=$(echo "${crate_max_stable_version}" | (grep --extended-regexp "^[0-9].*.[0-9].*.[0-9]$" || true))
    echo "${last_version}"
}

update_crate_version_in_toml() {
    # init args
    crate="$1"
    actual_version="$2"
    last_version="$3"
    toml_file="$4"

    # update crate version in toml
    sed -i -- "s/${crate} = { version = \"${actual_version}\"/${crate} = { version = \"${last_version}\"/g" "${toml_file}"
    sed -i -- "s/${crate} = \"${actual_version}\"/${crate} = \"${last_version}\"/g" "${toml_file}"
    if grep --extended-regexp --silent "${crate}.*=.*${last_version}" "${toml_file}"; then
        echo "${color_blue}updated to ${last_version}${color_reset}"
    else
        echo "${color_red}error, update to ${last_version} fails, please check ${toml_file} format and syntax, then re-run this script${color_reset}"
        return 1
    fi
}

which_is_the_newest_version() {
    # init vars
    first_version="$1"
    second_version="$2"

    # which is the newest version
    echo -e "${first_version}\n${second_version}" |
        sort --version-sort |
        tail -1
}

main() {
    # init vars
    crates_api_root_url="https://crates.io/api/v1/crates"
    arg="$1"
    check_args "${arg}"
    updated_count=0

    # update toml
    for package in packages/*; do
        toml_file="${package}/Cargo.toml"
        echo -e "\n=> crates updates for ${toml_file}\n"
        while read -r crate actual_version; do
            crate_url="${crates_api_root_url}/${crate}"
            last_version=$(get_crate_latest_version "${crate_url}")
            if [ -z "${last_version}" ]; then
                echo "${color_red}runtime error${color_reset}, i could not get last version from ${crate_url}"
                return 1
            fi
            echo -n "  ${crate} ${actual_version}: "
            newest_version=$(which_is_the_newest_version "${last_version}" "${actual_version}")
            if [ "${newest_version}" == "${actual_version}" ]; then
                echo "${color_green}newest${color_reset}"
            else
                if [ "${arg}" = "--check" ]; then
                    echo "${color_red}old, please update to latest ${last_version}${color_reset}"
                    updated_count=$((updated_count + 1))
                else
                    update_crate_version_in_toml "${crate}" "${actual_version}" "${last_version}" "${toml_file}"
                fi
            fi
        done < <(convert_toml_crates_to_key_value "${toml_file}")
    done
    wait

    # update lock
    if [ "${arg}" != "--check" ]; then
        echo -e "\n=> crates updates for Cargo.lock\n"
        cargo update --color always -vv
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
