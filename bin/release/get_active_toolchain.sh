#!/bin/bash
set -Eeuo pipefail

toolchain=$(rustup show active-toolchain | cut -d '-' -f 2- | cut -d ' ' -f1 | head -1)
echo "${toolchain}"

