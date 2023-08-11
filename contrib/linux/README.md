# Information on Linux Hurl Package

## Alpine

### Information

Site: <https://www.alpinelinux.org>

There is one Hurl package <https://pkgs.alpinelinux.org/package/edge/testing/x86_64/hurl>

### Contributing

Repo: <https://gitlab.alpinelinux.org/alpine/aports.git>

File: <https://git.alpinelinux.org/aports/tree/testing/hurl/APKBUILD>

Contributing is done through the GitLab Alpine: <https://gitlab.alpinelinux.org>


## Arch Linux

### Information

Site: <https://archlinux.org>

There are two Hurl packages in AUR user repository:

- <https://aur.archlinux.org/packages/hurl-bin>: the original ones, we contribute to it. This package uses the 
GitHub binaries released and do not try to build from source.
- <https://aur.archlinux.org/packages/hurl-rs>: a package that build Hurl from sources, including Integration tests. We 
should contact the package maintainers to simplify this package (mainly remove dependencies from python etc...)

### Contributing

Repo: <ssh://aur@aur.archlinux.org/hurl-bin.git> <https://aur.archlinux.org/hurl-bin.git> (read only)

Register an account: <https://aur.archlinux.org/register>

File: <https://aur.archlinux.org/cgit/aur.git/tree/PKGBUILD?h=hurl-bin>

File: <https://aur.archlinux.org/cgit/aur.git/tree/PKGBUILD?h=hurl-rs>

## Debian 

??

## NixOS

### Information

Site: <https://nixos.org>

### Contributing

Repo: <https://github.com/NixOS/nixpkgs>

File: <https://github.com/NixOS/nixpkgs/blob/nixos-unstable/pkgs/tools/networking/hurl/default.nix>