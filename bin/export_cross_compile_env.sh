#!/bin/bash
set -Eeuo pipefail

PKG_CONFIG_ALLOW_CROSS=1
CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc
CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++
export PKG_CONFIG_ALLOW_CROSS \
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER \
    CC_aarch64_unknown_linux_gnu \
    CXX_aarch64_unknown_linux_gnu

