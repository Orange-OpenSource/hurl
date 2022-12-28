#!/bin/bash
set -Eeuo pipefail

color_red=$(echo -e "\033[1;31m")
color_reset=$(echo -e "\033[0m")
errors_count=0

# Check *.rs Orange Copyright
echo "------------------------------------------------------------------------------------------"
while read -r rust_file ; do
    if [ "$(grep -c "Copyright (C) 2022 Orange" "$rust_file" || true)" -eq 0 ] ; then
        echo "Missing [Copyright (C) 2022 Orange] in ${color_red}${rust_file}${color_reset}"
        ((errors_count++))
    fi
done < <(find packages -type f -name "*.rs")

# Check *sh bash shebang at line 1
echo "------------------------------------------------------------------------------------------"
while read -r script ; do
    if [ "$(head -1 "$script" | grep -c "#!/bin/bash" || true)" -eq 0 ] ; then
        echo "Missing [#!/bin/bash] shebang in ${color_red}${script}${color_reset}"
        ((errors_count++))
    fi
done < <(find . -type f -name "*.sh")

# Check *sh error handling at line 2
echo "------------------------------------------------------------------------------------------"
while read -r script ; do
    if [ "$(head -2 "$script" | tail -1 | grep -c "set -Eeuo pipefail" || true)" -eq 0 ] ; then
        echo "Missing [set -Eeuo pipefail] in ${color_red}${script}${color_reset}"
        ((errors_count++))
    fi
done < <(find . -type f -name "*.sh")

# Check *PS1 error handling at line 2
echo "------------------------------------------------------------------------------------------"
while read -r script ; do
    if [ "$(head -1 "$script" | grep -c "Set-StrictMode -Version latest" || true)" -eq 0 ] ; then
        echo "Missing [Set-StrictMode -Version latest] in first line of ${color_red}${script}${color_reset}"
        ((errors_count++))
    fi
    if [ "$(head -2 "$script" | tail -1 | grep -c "\$ErrorActionPreference = 'Stop'" || true)" -eq 0 ] ; then
        echo "Missing [\$ErrorActionPreference = 'Stop'] in second line of ${color_red}${script}${color_reset}"
        ((errors_count++))
    fi
done < <(find . -type f -name "*.PS1")

# Control errors count
if [ "${errors_count}" -gt 0 ] ; then
    exit 1
fi

