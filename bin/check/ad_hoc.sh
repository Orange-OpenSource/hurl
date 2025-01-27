#!/bin/bash
set -Eeuo pipefail


color_red=$(echo -e "\033[1;31m")
color_green=$(echo -ne "\033[1;32m")
color_reset=$(echo -e "\033[0m")
errors_count=0

# Check *.rs Orange Copyright
echo "------------------------------------------------------------------------------------------"
while read -r rust_file ; do
    if [ "$(grep -c "Copyright (C) 2025 Orange" "$rust_file" || true)" -eq 0 ] ; then
        echo "Missing [Copyright (C) 2025 Orange] in ${color_red}${rust_file}${color_reset}"
        errors_count=$((errors_count+1))
    else
        echo "[Copyright (C) 2025 Orange] is present in ${color_green}${rust_file}${color_reset}"
    fi
done < <(find packages -type f -name "*.rs")

# Check *sh bash shebang at line 1
echo "------------------------------------------------------------------------------------------"
while read -r script ; do
    if [ "$(head -1 "$script" | grep -c "#!/bin/bash" || true)" -eq 0 ] ; then
        echo "Missing [#!/bin/bash] shebang in ${color_red}${script}${color_reset}"
        errors_count=$((errors_count+1))
    else
        echo "[#!/bin/bash] shebang is present in ${color_green}${script}${color_reset}"
    fi
done < <(find . -type f -name "*.sh")

# Check *sh error handling at first uncommented line
echo "------------------------------------------------------------------------------------------"
while read -r script ; do
    if [ "$(grep -Ev "^$|^#" "$script" | head -1 | grep -c "set -Eeuo pipefail" || true)" -eq 0 ] ; then
        echo "Missing [set -Eeuo pipefail] in ${color_red}${script}${color_reset}"
        errors_count=$((errors_count+1))
    else
        echo "[set -Eeuo pipefail] is present in ${color_green}${script}${color_reset}"
    fi
done < <(find . -type f -name "*.sh")

# Check bash function names in kebab case instead of camel case
echo "------------------------------------------------------------------------------------------"
while read -r script ; do
    kebab_case_function_list=$( (grep -Ev "^#" "${script}" || true) | (grep -E "^function" "${script}" || true) | cut --delimiter '{' --field 1 | cut --delimiter '(' --field 1 | tr -s ' ' | cut --delimiter ' ' --field 2)
    if [ -n "${kebab_case_function_list}" ] ; then
        while read -r function ; do
            if [[ "${function}" =~ "-" ]] ; then 
                echo "${color_red}${script}: function ${function}${color_reset} have to be: $(echo "${function}" | tr '-' '_')"
                errors_count=$((errors_count+1))
            else
                echo "${script}: function ${function} ${color_green}well formated${color_reset}"
            fi
        done < <(echo "${kebab_case_function_list}")
    fi
done < <(find . -type f -name "*.sh")

# Check *PS1 error handling at two first lines
echo "------------------------------------------------------------------------------------------"
while read -r script ; do
    if [ "$(head -1 "$script" | grep -c "Set-StrictMode -Version latest" || true)" -eq 0 ] ; then
        echo "Missing [Set-StrictMode -Version latest] in first line of ${color_red}${script}${color_reset}"
        errors_count=$((errors_count+1))
    else
        echo "[Set-StrictMode -Version latest] is present in first line of ${color_green}${script}${color_reset}"
    fi
    if [ "$(head -2 "$script" | tail -1 | grep -c "\$ErrorActionPreference = 'Stop'" || true)" -eq 0 ] ; then
        echo "Missing [\$ErrorActionPreference = 'Stop'] in second line of ${color_red}${script}${color_reset}"
        errors_count=$((errors_count+1))
    else
        echo "[\$ErrorActionPreference = 'Stop'] is present in second line of ${color_green}${script}${color_reset}"
    fi
done < <(find . -type f -name "*.ps1" | grep -v "./completions/")

# Check hurl command diffs between sh and ps1 tests files
echo "------------------------------------------------------------------------------------------"
tmp_sh="/tmp/sh"
tmp_ps1="/tmp/ps1"
tmp_diff="/tmp/diff"
touch "${tmp_sh}" "${tmp_ps1}" "${tmp_diff}"
command -v icdiff >/dev/null 2>&1 || sudo apt-get install -qq -y icdiff > /dev/null 2>&1
if tput cols >/dev/null 2>&1 ; then
    nb_cols="$(tput cols)"
else
    nb_cols=220
fi
function filter_hurl_and_hurlfmt { grep -E "hurl | hurl|hurlfmt | hurlfmt" "$1" || true ;}
function clean_indent { sed "s/^ *hurl/hurl/g" ;}
function uncomment { sed "s/^#//g" ;}
function clean_sh_var_redirect { sed "s/.*=.*(hurl/hurl/g" | sed "s/)$//g" ;}
function clean_ps1_var_redirect { sed "s/.*=hurl/hurl/g" ;}
function clean_c_drive { sed "s/C://g" ;}
function conv_ps1_antislash_to_sh { sed "s#\`\$#\\\#g" | sed "s#\`\\\#\\\\\\\#g" ;}
function conv_ps1_null_to_sh { sed "s#\$null#/dev/null#g" | sed "s#--output NUL#--output /dev/null#g" ;}
while read -r script_sh ; do
    script_ps1="${script_sh%.sh}.ps1"
    if [[ -f "${script_ps1}" ]] ; then
        filter_hurl_and_hurlfmt "${script_sh}" | clean_sh_var_redirect | clean_indent | uncomment > "${tmp_sh}"
        filter_hurl_and_hurlfmt "${script_ps1}" | clean_ps1_var_redirect | clean_c_drive | conv_ps1_antislash_to_sh | conv_ps1_null_to_sh | clean_indent | uncomment > "${tmp_ps1}"
        if ! cmp -s "${tmp_sh}" "${tmp_ps1}" >/dev/null 2>&1 ; then
            icdiff \
                --show-all-spaces \
                --highlight \
                --strip-trailing-cr \
                --cols="${nb_cols}" \
                --label="${script_sh}" \
                --label="${script_ps1}" \
                "${tmp_sh}" "${tmp_ps1}" | tee -a "${tmp_diff}"
            echo
            errors_count=$((errors_count+1))
        else
            echo "${script_sh} has the same hurl commands as ${color_green}${script_ps1}${color_reset}"
        fi
    else
        echo "${color_red}${script_sh}${color_reset} does not have his ${color_red}${script_ps1}${color_reset} clone."
        echo
        errors_count=$((errors_count+1))
    fi
done < <(find ./integration/hurl*/*/ -maxdepth 1 -type f -name "*sh" | sort)
unset -f filter_hurl_and_hurlfmt clean_indent uncomment clean_sh_var_redirect clean_ps1_var_redirect clean_c_drive conv_ps1_antislash_to_sh conv_ps1_null_to_sh

# Control errors count
if [ "${errors_count}" -gt 0 ] ; then
    exit 1
fi

