#!/bin/bash
set -Eeuo pipefail

get_sample_crate_version() {
  crate="$1"
  version=$(grep --regexp "^$crate = " contrib/sample/Cargo.toml | cut --delimiter ' ' --field 3 | sed 's/"//g')
  echo "$version"
}

get_snapshot_crate_version() {
  crate="$1"
  version=$(grep --regexp '^version = ' "packages/$crate/Cargo.toml" | cut --delimiter '"' --field 2)
  echo "$version"
}

get_major() {
  version="$1"
  major=$(echo "$version" | cut --delimiter '.' --field 1)
  echo "$major"
}

# Extract sample and snapshots versions
hurl_sample_version=$(get_sample_crate_version "hurl")
hurl_sample_major=$(get_major "$hurl_sample_version")
hurl_snapshot_version=$(get_snapshot_crate_version "hurl")
hurl_snapshot_major=$(get_major "$hurl_snapshot_version")
echo "hurl (sample)   major=$hurl_sample_major ($hurl_sample_version)"
echo "hurl (SNAPSHOT) major=$hurl_snapshot_major ($hurl_snapshot_version)"

hurl_core_sample_version=$(get_sample_crate_version "hurl_core")
hurl_core_sample_major=$(get_major "$hurl_core_sample_version")
hurl_core_snapshot_version=$(get_snapshot_crate_version "hurl_core")
hurl_core_snapshot_major=$(get_major "$hurl_core_snapshot_version")
echo "hurl_core (sample)   major=$hurl_core_sample_major ($hurl_core_sample_version)"
echo "hurl_core (SNAPSHOT) major=$hurl_core_snapshot_major ($hurl_core_snapshot_version)"

if [ "$hurl_sample_major" != "$hurl_snapshot_major" ] && [ "$hurl_core_sample_major" != "$hurl_core_snapshot_major" ];then
  echo "Major versions are different, no need to check crates compatibility"
  exit 0
fi

echo "Major versions are equal, check crates compatibility"

# Replace versions for our sample
sed -i -- "s/hurl = \"[0-9.]*\"/hurl = { version = \"$hurl_snapshot_version\", path = \"..\/..\/packages\/hurl\" }/" contrib/sample/Cargo.toml
sed -i -- "s/hurl_core = \"[0-9.]*\"/hurl_core = { version = \"$hurl_core_snapshot_version\", path = \"..\/..\/packages\/hurl_core\" }/" contrib/sample/Cargo.toml

cd contrib/sample
cargo clean
cargo update
cargo build
cargo run -- hello.hurl