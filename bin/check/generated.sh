#!/bin/bash
#!/bin/bash
set -Eeuo pipefail

cmds=(
    bin/spec/options/generate_all.py
)
for cmd in "${cmds[@]}"; do
    echo "$cmd"
    "$cmd"
    git diff --exit-code && exit_code=0 || exit_code=$?
    if [ "$exit_code" -ne 0 ]; then
        git --no-pager diff
        echo -e "You must regenerate files by running '$cmd'"
        echo "and commit them"
        exit 1
    fi
done
