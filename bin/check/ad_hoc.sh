#!/bin/bash
set -Eeuo pipefail

color_red=$(echo -e "\033[1;31m")
color_reset=$(echo -e "\033[0m")
errors_count=0

# Check *.rs Orange Copyright
echo "------------------------------------------------------------------------------------------"
find packages -type f -name "*.rs" | \
    while read -r rust_file ; do
        if [ $(grep -c "Copyright (C) 2022 Orange" "$rust_file" || true) -eq 0 ] ; then
            echo "Missing [Copyright (C) 2022 Orange] in ${color_red}${rust_file}${color_reset}"
            ((errors_count++))
        fi
    done

# Check *sh bash shebang at line 1
echo "------------------------------------------------------------------------------------------"
find . -type f -name "*.sh" | \
    while read -r script ; do
        if [ $(head -1 "$script" | grep -c "#!/bin/bash" || true) -eq 0 ] ; then
            echo "Missing [#!/bin/bash] shebang in ${color_red}${script}${color_reset}"
            ((errors_count++))
        fi
    done

# Check *sh error handling at line 2
echo "------------------------------------------------------------------------------------------"
find . -type f -name "*.sh" | \
    while read -r script ; do
        if [ $(head -2 "$script" | tail -1 | grep -c "set -Eeuo pipefail" || true) -eq 0 ] ; then
            echo "Missing [set -Eeuo pipefail] in ${color_red}${script}${color_reset}"
            ((errors_count++))
        fi
    done

# Control errors count
if [ "${errors_count}" -gt 0 ] ; then
    exit 1
fi
