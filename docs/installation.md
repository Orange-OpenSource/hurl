# Installation

## Binaries Installation

### Linux

Precompiled binary is available at [hurl-1.6.1-x86_64-linux.tar.gz]:

```shell
$ INSTALL_DIR=/tmp
$ curl -sL https://github.com/Orange-OpenSource/hurl/releases/download/1.6.1/hurl-1.6.1-x86_64-linux.tar.gz | tar xvz -C $INSTALL_DIR
$ export PATH=$INSTALL_DIR/hurl-1.6.1:$PATH
```

#### Debian / Ubuntu

For Debian / Ubuntu, Hurl can be installed using a binary .deb file provided in each Hurl release.

```shell
$ curl -LO https://github.com/Orange-OpenSource/hurl/releases/download/1.6.1/hurl_1.6.1_amd64.deb
$ sudo dpkg -i hurl_1.6.1_amd64.deb
```

#### Arch Linux / Manjaro

[`hurl-bin` package] for Arch Linux and derived distros is available via [AUR].

#### NixOS / Nix

[NixOS / Nix package] is available on stable channel.

### macOS

Precompiled binary is available at [hurl-1.6.1-x86_64-osx.tar.gz].

Hurl can also be installed with [Homebrew]:

```shell
$ brew install hurl
```

### Windows

#### Zip File

Hurl can be installed from a standalone zip file [hurl-1.6.1-win64.zip]. You will need to update your `PATH` variable.

#### Installer

An installer [hurl-1.6.1-win64-installer.exe] is also available.

#### Chocolatey

```shell
$ choco install hurl
```

#### Scoop

```shell
$ scoop install hurl
```

#### Windows Package Manager

```shell
$ winget install hurl
```

### Cargo

If you're a Rust programmer, Hurl can be installed with cargo.

```shell
$ cargo install hurl
```

### Docker

```shell
$ docker pull orangeopensource/hurl
```

### npm

```shell
$ npm install --save-dev @orangeopensource/hurl
```

## Building From Sources

Hurl sources are available in [GitHub].

### Build on Linux, macOS

Hurl depends on libssl, libcurl and libxml2 native libraries. You will need their development files in your platform.


#### Debian based distributions

```shell
$ apt install -y build-essential pkg-config libssl-dev libcurl4-openssl-dev libxml2-dev
```

#### Red Hat based distributions

```shell
$ yum install -y pkg-config gcc openssl-devel libxml2-devel
```

#### Arch based distributions

```shell
$ pacman -Sy --noconfirm pkgconf gcc openssl libxml2
```

#### macOS

```shell
$ xcode-select --install
$ brew install pkg-config
```

Hurl is written in [Rust]. You should [install] the latest stable release.

```shell
$ curl https://sh.rustup.rs -sSf | sh -s -- -y
$ source $HOME/.cargo/env
$ rustc --version
$ cargo --version
```

Then build hurl:

```shell
$ git clone https://github.com/Orange-OpenSource/hurl
$ cd hurl
$ cargo build --release
$ ./target/release/hurl --version
```

### Build on Windows

Please follow the [contrib on Windows section].

[GitHub]: https://github.com/Orange-OpenSource/hurl
[hurl-1.6.1-win64.zip]: https://github.com/Orange-OpenSource/hurl/releases/download/1.6.1/hurl-1.6.1-win64.zip
[hurl-1.6.1-win64-installer.exe]: https://github.com/Orange-OpenSource/hurl/releases/download/1.6.1/hurl-1.6.1-win64-installer.exe
[hurl-1.6.1-x86_64-osx.tar.gz]: https://github.com/Orange-OpenSource/hurl/releases/download/1.6.1/hurl-1.6.1-x86_64-osx.tar.gz
[hurl-1.6.1-x86_64-linux.tar.gz]: https://github.com/Orange-OpenSource/hurl/releases/download/1.6.1/hurl-1.6.1-x86_64-linux.tar.gz
[Homebrew]: https://brew.sh
[AUR]: https://wiki.archlinux.org/index.php/Arch_User_Repository
[`hurl-bin` package]: https://aur.archlinux.org/packages/hurl-bin/
[install]: https://www.rust-lang.org/tools/install
[Rust]: https://www.rust-lang.org
[contrib on Windows section]: https://github.com/Orange-OpenSource/hurl/blob/master/contrib/windows/README.md
[NixOS / Nix package]: https://search.nixos.org/packages?channel=21.11&from=0&size=1&sort=relevance&type=packages&query=hurl



