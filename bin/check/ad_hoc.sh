#!/bin/bash
set -Eeuo pipefail

color_red=$(echo -e "\033[1;31m")
color_green=$(echo -ne "\033[1;32m")
color_reset=$(echo -e "\033[0m")
errors_count=0

# Check *.rs Orange Copyright
echo "------------------------------------------------------------------------------------------"
while read -r rust_file ; do
    if [ "$(grep -c "Copyright (C) 2023 Orange" "$rust_file" || true)" -eq 0 ] ; then
        echo "Missing [Copyright (C) 2023 Orange] in ${color_red}${rust_file}${color_reset}"
        ((errors_count++))
    else
        echo "[Copyright (C) 2023 Orange] is present in ${color_green}${rust_file}${color_reset}"
    fi
done < <(find packages -type f -name "*.rs")

# Check *sh bash shebang at line 1
echo "------------------------------------------------------------------------------------------"
while read -r script ; do
    if [ "$(head -1 "$script" | grep -c "#!/bin/bash" || true)" -eq 0 ] ; then
        echo "Missing [#!/bin/bash] shebang in ${color_red}${script}${color_reset}"
        ((errors_count++))
    else
        echo "[#!/bin/bash] shebang is present in ${color_green}${script}${color_reset}"
    fi
done < <(find . -type f -name "*.sh")

# Check *sh error handling at first uncommented line
echo "------------------------------------------------------------------------------------------"
while read -r script ; do
    if [ "$(grep -Ev "^$|^#" "$script" | head -1 | grep -c "set -Eeuo pipefail" || true)" -eq 0 ] ; then
        echo "Missing [set -Eeuo pipefail] in ${color_red}${script}${color_reset}"
        ((errors_count++))
    else
        echo "[set -Eeuo pipefail] is present in ${color_green}${script}${color_reset}"
    fi
done < <(find . -type f -name "*.sh")

# Check *PS1 error handling at line 2
echo "------------------------------------------------------------------------------------------"
while read -r script ; do
    if [ "$(head -1 "$script" | grep -c "Set-StrictMode -Version latest" || true)" -eq 0 ] ; then
        echo "Missing [Set-StrictMode -Version latest] in first line of ${color_red}${script}${color_reset}"
        ((errors_count++))
    else
        echo "[Set-StrictMode -Version latest] is present in first line of ${color_green}${script}${color_reset}"
    fi
    if [ "$(head -2 "$script" | tail -1 | grep -c "\$ErrorActionPreference = 'Stop'" || true)" -eq 0 ] ; then
        echo "Missing [\$ErrorActionPreference = 'Stop'] in second line of ${color_red}${script}${color_reset}"
        ((errors_count++))
    else
        echo "[\$ErrorActionPreference = 'Stop'] is present in second line of ${color_green}${script}${color_reset}"
    fi
done < <(find . -type f -name "*.ps1")

# Check allowfailure tests flags expiration
echo "------------------------------------------------------------------------------------------"
max_expiration_in_days=60
actual_timestamp=$(date +%s)
while read -r file ; do
    file_touch_relative_age=$(git log -1 --format="%cr" -- "${file}")
    file_touch_timestamp=$(git log -1 --format="%ct" -- "${file}")
    max_expiration_in_seconds=$((max_expiration_in_days*24*60*60))
    file_age_in_seconds=$((actual_timestamp - file_touch_timestamp))
    if [ "${file_age_in_seconds}" -gt "${max_expiration_in_seconds}" ] ; then
        echo "${file} was created ${file_touch_relative_age}, ${color_red}it is older than the ${max_expiration_in_days} days allowed${color_reset}, please fix the according test and remove this flag"
        ((errors_count++))
    else
        echo "${file} was created ${file_touch_relative_age}, ${color_green}it is younger than the ${max_expiration_in_days} days allowed${color_reset}"
    fi
done < <(find ./integration -name "*.allowfailure")

# Control errors count
if [ "${errors_count}" -gt 0 ] ; then
    exit 1
fi

