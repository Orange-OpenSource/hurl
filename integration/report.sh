#!/bin/bash
# Generate a unique html report
# Each hurl file will be run successively, result is appended to the same json file
set +e
rm -rf build
mkdir build
export HURL_name=Bob

find tests -name "*.hurl" | sort | while read -r hurl_file; do
    options=("--report-html build/html" "--report-junit build/tests.xml" "--json" )
    if test -f "${hurl_file%.*}.options"; then
        options+=("$(cat "${hurl_file%.*}.options")")
    fi
    cmd="hurl $hurl_file ${options[*]}"
    echo "$cmd"
    $cmd >> "build/tests.json"
    exit_code=$?

    if [[ "$exit_code" != 0 && "$exit_code" != 3 && "$exit_code" != 4 ]]; then
	     echo "unexpected exit code $exit_code"
	     exit 1
    fi
done


total=$(ls tests/*.hurl | wc -l)
total_in_json=$(cat build/tests.json | wc -l)
total_in_xml=$(cat build/tests.xml | xmllint --xpath '//testcase' - | grep 'testcase id' | wc -l)

# Do not fail yet
echo "Total Number of tests"
echo "Hurl files:    $total"
echo "Tests in JSON: $total_in_json"
echo "Tests in XML:  $total_in_xml"


