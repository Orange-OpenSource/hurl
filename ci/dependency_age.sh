#!/bin/bash

exit_code=0
color_red=$(echo -ne "\033[1;31m")
color_green=$(echo -ne "\033[1;32m")
color_reset=$(echo -ne "\033[0m")

for package in packages/* ; do
  echo -e "\n=> dependency age for ${package}\n"
  while read -r dependency actual_version ; do
    last_version=$(curl -v "https://docs.rs/${dependency}" 2>&1 | grep "< location:" | cut --delimiter "/" --field 5 | grep -E "^[0-9].*.[0-9].*.[0-9]$")
    if [ -z "${last_version}" ] ; then
      echo "${color_red}runtime error${color_reset}, i could not get last version from https://docs.rs/${dependency}"
      exit 1
    fi

    echo -n "  ${dependency} ${actual_version}: "
    newest_version=$(echo -e "${last_version}\n${actual_version}" | sort -V | tail -1)
    if [ "${newest_version}"  == "${actual_version}" ] ; then
      echo "${color_green}newest${color_reset}"
    else
      echo "${color_red}old, please update to latest ${last_version}${color_reset}"
      ((exit_code++))
    fi

  done < <(sed -n "/dependencies\]/,/^$/p" "${package}/Cargo.toml" | grep -Ev "\[|path|^$" | tr -d '" ' | tr '=' ' ')
done
exit $exit_code
