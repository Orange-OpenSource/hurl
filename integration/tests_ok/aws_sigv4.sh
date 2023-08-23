#!/bin/bash
set -uo pipefail

# FIXME: remove this workaround once all integration test targets have aws-sigv4 support in libcurl
#
# for integration test targets that come with a too old libcurl, we accept the appropriate error
# message and fake the correct result (but only if `curl` doesn't know about `--aws-sigv4` either!).

output_curl=$(curl --aws-sigv4 2>&1)
output_hurl=$(hurl --user someAccessKeyId:someSecretKey tests_ok/aws_sigv4.hurl 2>&1 )
rc="$?"

if echo "$output_curl" | grep -q 'option --aws-sigv4: is unknown'; then
	# curl on this system does not support --aws-sigv4, so check for the expected error message

	if echo "$output_hurl" | grep -q "Option aws-sigv4 requires libcurl version 7.75.0 or higher"; then
		cat tests_ok/aws_sigv4.out
		exit 0
	fi
fi

echo -n "$output_hurl"
exit "$rc"

