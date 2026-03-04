#!/bin/bash
set -Eeuo pipefail

color_red=$(echo -ne "\033[1;31m")
color_reset=$(echo -ne "\033[0m")

gh repo set-default --view
if gh repo set-default --view 2>&1 | grep "No default" >/dev/null 2>&1 ; then
    gh repo set-default origin
fi
gh repo set-default --view

dev_version=$(grep '^version' packages/hurl/Cargo.toml | cut -f2 -d'"')
dev_major=$(echo "${dev_version}" | cut -d'.' -f1)
latest_release_version=$(gh release list --limit 1 --json tagName --jq '.[0].tagName' 2>/dev/null | sed 's/^v//g' || true)
latest_release_major=$(echo "${latest_release_version}" | cut -d'.' -f1)
echo "- Dev version: ${dev_version}, Dev major: $dev_major"
echo "- Last released version: ${latest_release_version}, Last released major: $latest_release_major"

if [[ -z "${latest_release_version}" ]]; then
    echo "${color_red}- ERROR:${color_reset} I can not retrieve latest release version on GitHub, please check logs"
    exit 1
fi
if [[ "${dev_major}" -gt "${latest_release_major}" ]]; then
    echo "> Skipping semver check as actual major version is higher than latest release major version"
    exit 0
fi

install_dependencies="${1:-}"
install_dependencies_option_name="--install-dependencies"
if ! cargo semver-checks --version >/dev/null 2>&1 ; then
    if [ -n "$install_dependencies" ] && [ "$install_dependencies" == "$install_dependencies_option_name" ] ; then
        echo "------------------------------------------------"
        echo "- Installing cargo-semver-checks"
        cargo install cargo-semver-checks --locked
        echo "------------------------------------------------"
    else
        echo "------------------------------------------------"
        echo "${color_red}cargo-semver-checks is not installed${color_reset}, please install it manually or re-run this script with $install_dependencies_option_name"
        exit 1
    fi
fi

result=/tmp/output.txt
echo "------------------------------------------------"
echo "- run semver check"
cargo semver-checks > "${result}" 2>&1 && exit_code=0 || exit_code=$?
cat $result
if grep -i "unsupported rustdoc format" "${result}" >/dev/null 2>&1 ; then
    echo "> Allowing \"unsupported rustdoc format\" failure because actual rustdoc format is not supported by cargo-semver-checks for now"
    exit 0
fi
exit "${exit_code}"

