#!/bin/bash
set -Eeuo pipefail

# First generates Rust code, bash completion, part of hurl.md and hurlfmt.md
python3 bin/spec/options/generate_all.py

# Generates manual, READMEs etc..
python3 bin/release/gen_manpage.py docs/manual/hurl.md > docs/manual/hurl.1
python3 bin/release/gen_manpage.py docs/manual/hurlfmt.md > docs/manual/hurlfmt.1
python3 bin/docs/build_man_md.py docs/manual/hurl.md > docs/manual.md
python3 bin/docs/build_readme.py github > README.md
python3 bin/docs/build_readme.py crates > packages/hurl/README.md

# Generates standalone doc
VERSION="$(grep '^version' packages/hurl/Cargo.toml | cut -f2 -d'"')"
VERSION=${VERSION%-SNAPSHOT}
python3 bin/docs/build_standalone_md.py > docs/standalone/hurl-"$VERSION".md
