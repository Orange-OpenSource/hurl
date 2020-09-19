#!/bin/bash
set -e

shellcheck ./*.sh ./*/*.sh

# watch eprintln!/eprintln!
find src -name "*.rs" | grep -E -v 'logger|hurlfmt|http/client' | while read -r f; do
  if grep -q eprintln "$f"; then
      echo "file '$f' contains a println!"
      exit 1
  fi


done
