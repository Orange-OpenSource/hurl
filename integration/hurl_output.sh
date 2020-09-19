#!/bin/bash

for command_file in "$@"; do
    echo "$command_file"
    command=$(cat "$command_file")
    expected=$(cat "${command_file%.*}.output")
    output="$($command 2>&1)"

   if [ "$output" != "$expected" ]; then
        diff  <(echo "$output" ) <(echo "$expected")
        exit 1
    fi

done
