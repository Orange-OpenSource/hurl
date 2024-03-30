#!/bin/bash
set -Eeuo pipefail

# functions
function init_terminal_colors(){
    color_red=$(echo -ne "\033[1;31m")
    color_yellow=$(echo -ne "\033[1;33m")
    color_reset=$(echo -ne "\033[0m")
}

function log(){
    color="$1"
    title="$2"
    message="$3"
    if basename "$0" >/dev/null 2>&1 ; then
        parent=$(basename "$0")
    else
        parent="."
    fi
    echo "${color}${parent}: ${title}: ${message}${color_reset}" 1>&2
}

function log_error(){
    title="$1"
    message="$2"
    log "❌ ${color_red}" "${title}" "${message}"
}

function prerequisites(){
if ! command -v 7z >/dev/null 2>&1 ; then
    log_error "prerequisite" "Please install p7zip-full to make $(basename "$0") work."
    return 1
elif ! command -v tar >/dev/null 2>&1 ; then
    log_error "prerequisite" "Please install tar to make $(basename "$0") work."
    return 1
elif ! command -v unzip >/dev/null 2>&1 ; then
    log_error "prerequisite" "Please install unzip to make $(basename "$0") work."
    return 1
elif ! command -v icdiff >/dev/null 2>&1 ; then
    log_error "prerequisite" "Please install icdiff to make $(basename "$0") work."
    return 1
fi
}

function usage(){
    echo
    echo "Description: Displays and checks package anatomy for deb, tar.gz, zip and nsis formats, therefore securing what is delivered."
    echo "             Package anatomy is composed of the semver pattern, elements count and files tree" 
    echo
    echo "Dependencies: p7zip-full, tar, gzip, icdiff"
    echo
    echo
    echo "Usage: $(basename "$0") <package file> [Options]..."
    echo
    echo "   Example: this command outputs hurl_4.2.0_amd64.deb anatomy"
    echo
    echo "   $ $(basename "$0") hurl_4.2.0_amd64.deb"
    echo
    echo "Options: #mandatory #optional"
    echo
    echo "  --help #optional"
    echo
    echo "      This help text."
    echo
    echo "  --output <anatomy file> #optional"
    echo
    echo "      Write standard output to <anatomy file>. Not compatible with --compare-with, --compare-with-dir and --output-dir options."
    echo "      Example: This command creates hurl_deb_package.anatomy file."
    echo
    echo "      $ $(basename "$0") hurl_4.2.0_amd64.deb --output hurl_deb_package.anatomy"
    echo
    echo "  --output-dir <anatomy files dir> #optional"
    echo
    echo "      Write standard output to the standard anatomy file <semver pattern>.anatomy in <dir>. Not compatible with --compare-with, --compare-with-dir and --output options."
    echo "      Example: This command creates ./anatomy_dir/hurl_x.y.z_amd64.deb.anatomy file."
    echo
    echo "      $ $(basename "$0") hurl_4.2.0_amd64.deb --output-dir ./anatomy_dir"
    echo
    echo "  --compare-with <anatomy file> #optional"
    echo
    echo "      Compare <package file> anatomy with <anatomy file> and fails if there are differences. Not compatible with --output, --output-dir and --compare-with-dir options."
    echo "      Example: This command gets hurl_5.0.0_amd64.deb anatomy and compares it with hurl_deb_package.anatomy reference."
    echo
    echo "      $ $(basename "$0") hurl_5.0.0_amd64.deb --compare-with hurl_deb_package.anatomy"
    echo
    echo "  --compare-with-dir <dir> #optional"
    echo
    echo "      Compare <package file> anatomy with standard anatomy file <semver pattern>.anatomy in <dir> and fails if there are differences. Not compatible with --output, --output-dir and --compare-with options."
    echo "      Example: This command gets hurl_5.0.0_amd64.deb anatomy and compares it with hurl_deb_package.anatomy reference."
    echo
    echo "      $ $(basename "$0") hurl_5.0.0_amd64.deb --compare-with hurl_deb_package.anatomy"
    echo
}

function consume_args(){
    # iterate over args
    while [[ $# -gt 0 ]] ; do
        case "$1" in
        --help)
            usage
            exit 0
            ;;
        --output)
            if [[ -n ${2:-} ]] ; then
                output="$2"
                shift
                shift
            else
                log_error "option $1" "Can not be null."
                usage >&2
                return 1
            fi
            ;;
        --output-dir)
            if [[ -n ${2:-} ]] ; then
                output_dir="$2"
                shift
                shift
            else
                log_error "option $1" "Can not be null."
                usage >&2
                return 1
            fi
            ;;
        --compare-with)
            if [[ -n ${2:-} ]] ; then
                compare_with="$2"
                shift
                shift
            else
                log_error "option $1" "Can not be null."
                usage >&2
                return 1
            fi
            ;;
        --compare-with-dir)
            if [[ -n ${2:-} ]] ; then
                compare_with_dir="$2"
                shift
                shift
            else
                log_error "option $1" "Can not be null."
                usage >&2
                return 1
            fi
            ;;
        --*)
            log_error "option $1" "Is unknown."
            usage >&2
            return 1
            ;;
        *)
            file=${file:-}
            if [[ -z ${file} ]] ; then
                file="$1"
                shift
            else
                shift
            fi
        esac
    done
    # check mandatory options
    file=${file:-}
    if [[ -z $file ]] ; then
        log_error "option --file" "Is mandatory."
        usage >&2
        return 1
    fi
    # check options incompatibilities
    output=${output:-}
    output_dir=${output_dir:-}
    compare_with=${compare_with:-}
    compare_with_dir=${compare_with_dir:-}
    incompatible_options_count=0
    [[ $output != "" ]] && incompatible_options_count=$((incompatible_options_count+1))
    [[ $output_dir != "" ]] && incompatible_options_count=$((incompatible_options_count+1))
    [[ $compare_with != "" ]] && incompatible_options_count=$((incompatible_options_count+1))
    [[ $compare_with_dir != "" ]] && incompatible_options_count=$((incompatible_options_count+1))
    if [[ ${incompatible_options_count} -gt 1 ]] ; then
        log_error "options incompatibility" "You can not use --output, --output-dir, --compare-with and --compare-with-dir together."
        usage >&2
        return 1
    fi
}

function test_file(){
    file="$1"
    if ! echo "${file}" | grep -E ".deb$|tar.gz$|.zip$|.exe$" >/dev/null ; then
        log_error "file" "I can't anatomize ${file} because extension is not one of .deb, .tar.gz, .exe, or .zip"
        return 1
    fi
    if ! [[ -e ${file} ]] ; then
        log_error "file" "I can't anatomize ${file} because it does not exist or i don't have read permissions"
        return 1
    fi
    if ! [[ -f ${file} ]] ; then
        what=$(file "${file}" | cut --delimiter ":" --field 2-)
        log_error "file" "I can't anatomize ${file} because it's not a file, it's a ${what}"
        return 1
    fi
}

function format_tree(){
    raw_tree="$1"
    raw=$(
    echo "attr    user_group type file"
    echo "------- ---------- ---- ----"
    while read -r attr user_group type file ; do
        if [[ ${file} =~ /$ ]] ; then
            type="dir"
            elif [[ ${type} -eq 0 ]] ; then
                type="empty-file"
            else
                type="file"
            fi
            echo "${attr} ${user_group} ${type} $(anonymize_semver "${file}")"
        done <<< "$raw_tree" | sort -k 4,4
    )
    echo "${raw}" | column -t
}

function tree_file(){
    file="$1"
    if [[ ${file} =~ \.deb$ ]] ; then
        if ! (7z l "${file}" 2>/dev/null | grep data.tar >/dev/null 2>&1) ; then
            log_error "package integrity" "${file} is not a valid deb package as it does not contain a root file named data.tar"
            return 1
        fi
        if ! (7z e -so "${file}" data.tar 2>/dev/null | tar -tvf - >/dev/null 2>&1) ; then
            log_error "package integrity" "${file} is not a valid deb package as his data.tar file is not a valid tar package"
            return 1
        fi
        raw_tree=$(tree_deb "${file}")
    elif [[ ${file} =~ \.zip$ ]] ; then
        if ! unzip -t "${file}" >/dev/null 2>&1 ; then
            log_error "package integrity" "${file} is not a valid zip package"
            return 1
        fi
        raw_tree=$(tree_zip "${file}")
    elif [[ ${file} =~ \.tar.gz$ ]] ; then
        if ! tar -tzf "${file}" >/dev/null 2>&1 ; then
            log_error "package integrity" "${file} is not a valid tar.gz package"
            return 1
        fi
        raw_tree=$(tree_tar_gz "${file}")
    elif [[ ${file} =~ \.exe$ ]] ; then
        if ! (7z l "${file}" 2>/dev/null | grep "Type = Nsis" >/dev/null 2>&1 ) ; then
            log_error "package integrity" "${file} exe package is not a valid nsis installer"
            return 1
        fi
        raw_tree=$(tree_nsis "${file}")
    fi
    format_tree "${raw_tree}"
}

function tree_deb(){
    file="$1"
    raw_tree=$(7z e -so "${file}" data.tar | tar -tvf - | grep -vE "\./$" | tr -s ' ' | cut --delimiter " " --field 1,2,3,6)
    echo "${raw_tree}"
}

function tree_zip(){
    file="$1"
    raw_tree=$(unzip -Z "${file}" | tr -s ' ' | grep -Ev "^Archive|^Zip|bytes compressed" | tr -s ' ' | cut --delimiter " " --field 1,4,6,9)
    while read -r attr size user_group file ; do
        echo "${attr} ${user_group} ${size} ${file}"
    done <<< "${raw_tree}"
}

function tree_tar_gz(){
    file="$1"
    raw_tree=$(tar -tvf "${file}" | grep -vE "\./$" | tr -s ' ' | cut --delimiter " " --field 1,2,3,6)
    echo "${raw_tree}"
}

function tree_nsis(){
    file="$1"
    raw_tree=$(7z l -ba "${file}" | sed "s/^ /---------- --:--:--/" | tr -s ' ' | rev | cut --delimiter " " --field 1,2 | rev | sed "s/^/------- --- /g")
    dirs=$(echo "${raw_tree}" | grep "/" | rev | cut --delimiter "/" --field 2- | cut --delimiter " " --field 1 | rev | sort -u)
    for dir in ${dirs} ; do
        echo "------- --- 0 ${dir}/"
    done
    echo "${raw_tree}"
}

function validate_anatomy_file(){
    anatomy_file="$1"
    if ! [[ -f ${anatomy_file} ]] ; then
        log_error "validate anatomy file" "${anatomy_file} does not exists or I don't avec permission to read it." 
        return 1
    elif ! grep "semver pattern:" "${compare_with}" >/dev/null 2>&1 ; then
        log_error "validate anatomy file" "${anatomy_file} is not an anatomy file as it does not have expected headers"
        return 1
    fi
}

function anonymize_semver(){
    string="$1"
    string_without_snapshot="${string//-SNAPSHOT/}"
    anonymized_semver="${string_without_snapshot//[0-9]\.[0-9]\.[0-9]/x\.y\.z}"
    echo "${anonymized_semver}"
}

function semver_pattern(){
    file="$1"
    basename=$(basename "${file}")
    semver_pattern=$(anonymize_semver "${basename}")
    echo "${semver_pattern}"
}

function anatomize(){
    file="$1"
    tree="$2"
    elements_count=$(echo "${tree}" | wc -l)
    dirs_count=$(echo "${tree}" | (grep -Ec ".* dir .*" || true) )
    files_count=$(echo "${tree}" | (grep -Ec ".* file .*" || true) )
    empty_files_count=$(echo "${tree}" | (grep -Ec ".* empty-file .*" || true) )
    semver_pattern=$(semver_pattern "${file}")
    echo "[semver pattern: ${semver_pattern}]"
    echo "[elements: total ${elements_count}, dirs ${dirs_count}, files ${files_count}, empty-files ${empty_files_count}]"
    echo
    echo "${tree}"
}

# main
init_terminal_colors
prerequisites
consume_args "$@"
test_file "${file}"
tree=$(tree_file "${file}")
if [[ -n ${compare_with} ]] || [[ -n ${compare_with_dir} ]] ; then
    if [[ -n ${compare_with_dir} ]] ; then
        compare_with="${compare_with_dir}/$(semver_pattern "${file}").anatomy"
    fi
    validate_anatomy_file "${compare_with}"
    file_anatomy=$(anatomize "${file}" "${tree}")
    compare_with_anatomy=$(cat "${compare_with}")
    if diff -q <(echo "${file_anatomy}") <(echo "${compare_with_anatomy}") >/dev/null 2>&1 ; then
        echo "✅ no diffs between ${file} anatomy and ${compare_with} ref file"
        echo
    else
        echo "❌ ${file} anatomy is different from ${compare_with} anatomy:" 1>&2
        echo 1>&2
        if tput cols >/dev/null 2>&1 ; then
            nb_cols="$(tput cols)"
        else
            nb_cols=220
        fi
        icdiff \
            --show-all-spaces \
            --line-numbers \
            --highlight \
            --strip-trailing-cr \
            --cols="${nb_cols}" \
            --label="${file}" \
            --label="${compare_with}" \
            <(echo "${file_anatomy}") <(echo "${compare_with_anatomy}") 1>&2
        echo 1>&2
        echo "🔔 ${color_yellow}If you want to fix it just run:${color_reset}" 1>&2
        echo 1>&2
        echo "    $ $(basename "$0") ${file} --output ${compare_with}" 1>&2
        echo 1>&2
        echo
        exit 1
    fi
elif [[ -n ${output} ]] || [[ -n ${output_dir} ]] ; then
    if [[ -n ${output} ]] ; then
        output_dirname=$(dirname "${output}")
    elif [[ -n ${output_dir} ]] ; then
        output_dirname=${output_dir}
        output="${output_dirname}/$(semver_pattern "${file}").anatomy"
    fi
    mkdir -p "${output_dirname}" || true
    if ! [[ -d ${output_dirname} ]] ; then
        log_error "permission denied" "Can not create dir ${output_dirname}."
        exit 1
    fi
    anatomize "${file}" "${tree}" > "${output}"
else
    anatomize "${file}" "${tree}"
fi

