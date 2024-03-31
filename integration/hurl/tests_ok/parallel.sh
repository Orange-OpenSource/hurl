#!/bin/bash
set -Eeuo pipefail
rm -f build/parallel.tap

# We run 4 Hurl files in parallel, each one has a ~5s duration.
# On usual hardware, this should be executed in ~5s.

start=$(date +%s)

hurl --parallel --jobs 4 --verbose --variable name=Bob \
  tests_ok/parallel.hurl \
  tests_ok/parallel.hurl \
  tests_ok/parallel.hurl \
  tests_ok/parallel.hurl

end=timestamp=$(date +%s)

duration=$((end-start))
if ((duration > 6)); then
  echo "Parallel execution duration failed ${duration}s (limit 6s)"
  exit 1
fi

