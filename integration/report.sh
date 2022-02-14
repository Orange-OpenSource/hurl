#!/bin/bash
# Generate a unique html report
# Each hurl file will be run successively, result is appended to the same json file
set +e
rm -rf build
mkdir build
export HURL_name=Bob

# read options
# params:
#     options_file   optional filename
function read_options() {
  file=$1
  if test -f "$file"; then
    while read -r option ; do
     if [[ $option =~ " " ]]; then
        echo -n " '$option'"
     else
         echo -n " $option"
     fi
     done < "$options_file"
  fi

}

find tests_{ok,failed} -name "*.hurl" | sort | while read -r hurl_file; do
    options_file="${hurl_file%.*}.options"
    options="--report-html build/html --report-junit build/tests.xml --json $(read_options "$options_file")"
    cmd="hurl $hurl_file $options"
    echo "$cmd"
    echo "$cmd" | sh >> "build/tests.json"
    exit_code=$?
    if [[ "$exit_code" != 0 && "$exit_code" != 3 && "$exit_code" != 4 ]]; then
	     echo "unexpected exit code $exit_code"
	     exit 1
    fi
done

set -e

total=$(find tests_{ok,failed} -name '*.hurl' | wc -l)
total_in_json=$( wc -l < build/tests.json)
total_in_xml=$(xmllint --xpath '//testcase' - < build/tests.xml| grep -c 'testcase id')

echo "Total Number of tests: $total"

if [[ "$total_in_json" -ne "$total" ]] ; then
  echo "Number of tests in JSON do not match!"
  exit 1
fi
if [[ "$total_in_xml" -ne "$total" ]] ; then
  echo "Number of tests in XML do not match!"
  exit 1
fi




