#!/bin/bash
set -u

EXITCODE_EXPECTED=1

for hurl_file in "$@"; do
    echo "$hurl_file";
    set +e
    hurlfmt --color --check "$hurl_file" 2>/tmp/test.stderr

    EXITCODE_ACTUAL=$?
    set -e
    if [ "$EXITCODE_ACTUAL" != "$EXITCODE_EXPECTED" ]; then
        echo "ERROR Exit Code"
        echo "  Expected: $EXITCODE_EXPECTED"
        echo "  Actual: $EXITCODE_ACTUAL"
        exit 1
    fi

    STDERR_ACTUAL=$(cat /tmp/test.stderr)
    STDERR_EXPECTED=$(cat ${hurl_file%%.*}.err)
    diff ${hurl_file%%.*}.err /tmp/test.stderr
    if [ "$STDERR_ACTUAL" != "$STDERR_EXPECTED" ]; then
        echo "ERROR stderr"
        echo "  expected:"
        echo "$STDERR_EXPECTED" |  perl -pe 'chomp;s/.*/    $_\n/'
        echo "  actual:"
        echo "$STDERR_ACTUAL"  |  perl -pe 'chomp;s/.*/    $_\n/'
        exit 1
    fi

    hurlfmt "$hurl_file" --no-color >/tmp/test.lint
    LINT_ACTUAL=$(cat /tmp/test.lint)
    LINT_EXPECTED=$(cat ${hurl_file%%.*}.hurl.lint)
    if [ "$LINT_ACTUAL" != "$LINT_EXPECTED" ]; then
        echo "ERROR linting"
        echo "  expected:"
        echo "$LINT_EXPECTED" |  perl -pe 'chomp;s/.*/    $_\n/'
        echo "  actual:"
        echo "$LINT_ACTUAL"  |  perl -pe 'chomp;s/.*/    $_\n/'
        exit 1
    fi

done


