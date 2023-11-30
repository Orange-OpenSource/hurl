#!/bin/bash
set -Eeuo pipefail

# functions

function init_terminal_colors(){
    color_black=$(echo -ne "\033[1;30m")
    color_red=$(echo -ne "\033[1;31m")
    color_green=$(echo -ne "\033[1;32m")
    color_yellow=$(echo -ne "\033[1;33m")
    color_blue=$(echo -ne "\033[1;34m")
    color_purple=$(echo -ne "\033[1;34m")
    color_cyan=$(echo -ne "\033[1;36m")
    color_backwhite=$(echo -ne "\033[1;47m")
    color_reset=$(echo -ne "\033[0m")
    export color_black color_red color_green color_yellow color_blue color_purple color_cyan color_backwhite color_reset
}

function print_error(){
    title=$1
    message=$2
    echo "${color_red}$(basename "$0"): ${title}: ${message}${color_reset}"
}

