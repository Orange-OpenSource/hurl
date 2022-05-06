#!/bin/bash

exit_code=0
color_red=$(echo -ne "\033[1;31m")
color_green=$(echo -ne "\033[1;32m")
color_yellow=$(echo -ne "\033[1;33m")
color_blue=$(echo -ne "\033[1;34m")
color_reset=$(echo -ne "\033[0m")

for package in packages/* ; do
  echo -e "\n=> dependency age for ${package}\n"
  while read -r dependency actual_version ; do
    last_version=$(curl -kLs "https://crates.io/api/v1/crates/${dependency}" | jq -r .crate.max_stable_version | grep --extended-regexp "^[0-9].*.[0-9].*.[0-9]$")
    if [ -z "${last_version}" ] ; then
      echo "${color_red}runtime error${color_reset}, i could not get last version from https://docs.rs/${dependency}"
      exit 1
    fi

    echo -n "  ${dependency} ${actual_version}: "
    newest_version=$(echo -e "${last_version}\n${actual_version}" | sort --version-sort | tail -1)
    if [ "${newest_version}"  == "${actual_version}" ] ; then
      if [ "$1" = "--update" ] ; then
        echo "${color_green}newest, nothing to update${color_reset}"
      else
        echo "${color_green}newest${color_reset}"
    fi 
    else
      if [ "$1" = "--update" ] ; then
        sed -i" " "s/${dependency} = { version = \"${actual_version}\"/${dependency} = { version = \"${last_version}\"/g" "${package}/Cargo.toml"
        sed -i" " "s/${dependency} = \"${actual_version}\"/${dependency} = \"${last_version}\"/g" "${package}/Cargo.toml"
	grep --extended-regexp --silent "${dependency}.*=.*\"${last_version}\"" "${package}/Cargo.toml" && \
	echo "${color_blue}old, updated to ${last_version}${color_reset}" || \
	echo "${color_red}error, update to ${last_version} fails, please check ${package}/Cargo.toml format and syntax${color_reset}"
      else
        echo "${color_red}old, please update to latest ${last_version}${color_reset}"
      fi
      ((exit_code++))
    fi

  done < <(sed -n "/dependencies\]/,/^$/p" "${package}/Cargo.toml" | grep --extended-regexp --invert-match "^\[|path|^$" | cut --delimiter ',' --field 1 | sed "s/version//g" | tr -d '"{=' | tr -s ' ')
done

if [ "$1" != "--update" ] && [ "${exit_code}" -gt 0 ] ; then
  echo -e "\n${color_yellow}...Consider executing \"dependency_age.sh --update\" to automatically update ${exit_code} old dependencies on your Cargo.toml files${color_reset}"
fi
exit "$exit_code"
