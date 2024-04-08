#!/bin/bash
set -Eeuo pipefail

# functions
function init_terminal_colors(){
    color_red=$(echo -ne "\033[1;31m")
    color_green=$(echo -ne "\033[1;32m")
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

function usage(){
    echo "Description: Prints sha256sum for input files, and write it to <input dir file>/<input file>.sha256."
    echo
    echo "Usage: $(basename "$0") [Options]... <file1> <file2>..."
    echo "   Example: This command prints sha256sum for input files"
    echo "            $ $(basename "$0") file1.zip file2.exe file3.tar.gz"
    echo
    echo "Options: #mandatory #optional"
    echo
    echo "  --help #optional"
    echo "      This help text"
    echo
    echo "  --write #optional"
    echo "      Write sha256sum to <input dir file>/<input file>.sha256"   
    echo "        : default value: false"
    echo "        : example: $(basename "$0") --write dir1/file1.zip dir2/file2.exe dir3/dir3/file3.tar.gz"
    echo "                   Will print sha256sum to standard output and create dir1/file1.zip.sha256 dir2/file2.exe.sha256 dir3/dir3/file3.tar.gz.sha256"
    echo
}

function consume_args(){
    files=''
    write=false
    while [[ $# -gt 0 ]] ; do
        case "$1" in
        --help)
            usage
            exit 0
            ;;
        --write)
            if [[ ${2:-} =~ true|false ]] ; then
                write=$2
                shift
                shift
            else
                write=true
                shift
            fi
            ;;
        --*)
            log "$color_red" "option $1" "is unknown"
            usage >&2
            return 1
            ;;
        *)
            file="$1"
            files="$files $1"
            shift
        esac
    done

    # check mandatory options
    if [[ -z $files ]] ; then
        log "$color_red" "mandatory option" "Please specify an input file"
        usage
        exit 1
    fi
}

# main
init_terminal_colors
consume_args "$@"
for file in $files ; do
    if [[ -f $file ]] ; then
        sha256sum_file="$file.sha256"
        sha256sum=$(sha256sum "$file")
        if [[ $write == false ]] ; then
            echo "${sha256sum}"
        else
            echo -n "${sha256sum},"
            echo "${sha256sum}" | cut --delimiter ' ' --field 1 > "$sha256sum_file" && exit_code=0 || exit_code=1
            if [[ $exit_code -eq 0 ]] ; then
                echo "    $sha256sum_file ${color_green}created${color_reset}"
            else
                echo
                log "$color_red" "output permissions" "Can not output to $sha256sum_file"
                exit 1
            fi
        fi
    else
        log "$color_red" "input file" "$file does not exist"
        exit 1
    fi
done
