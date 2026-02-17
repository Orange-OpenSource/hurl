#!/bin/bash
set -Eeuo pipefail

# test is KO but we want the script to continue until the end
set +eo pipefail
rm -f build/stderr_piped.txt
hurl tests_pty/stderr/stderr.hurl 2>build/stderr_piped.txt
set -Eeuo pipefail

# On Fedora, when using Hurl generic package, we may have a warning about missing version information
# We filter it as it's not really relevant.
sed -i.bak '/libcurl\.so\.4: no version information available/d' build/stderr_piped.txt
cat build/stderr_piped.txt
