#!/bin/bash
set -Eeuo pipefail

echo "## create docs:"
cat >copyright <<END
Files: *
Copyright: 2023, Orange
License: http://www.apache.org/licenses/LICENSE-2.0
END
gzip -9 -n --stdout CHANGELOG.md > changelog.Debian.gz
ls -l copyright
ls -l changelog.Debian.gz
