# Installation

## Binaries Installation

### Linux

Precompiled binary is available at [Hurl latest GitHub release]:

```shell
$ INSTALL_DIR=/tmp
$ VERSION=4.2.0
$ curl --silent --location https://github.com/Orange-OpenSource/hurl/releases/download/$VERSION/hurl-$VERSION-x86_64-unknown-linux-gnu.tar.gz | tar xvz -C $INSTALL_DIR
$ export PATH=$INSTALL_DIR/hurl-$VERSION:$PATH
```

#### Debian / Ubuntu

For Debian / Ubuntu, Hurl can be installed using a binary .deb file provided in each Hurl release.

```shell
$ VERSION=4.2.0
$ curl --location --remote-name https://github.com/Orange-OpenSource/hurl/releases/download/$VERSION/hurl_${VERSION}_amd64.deb
$ sudo apt update && sudo apt install ./hurl_${VERSION}_amd64.deb
```

#### Alpine

Hurl is available on `testing` channel.

```shell
$ apk add --repository http://dl-cdn.alpinelinux.org/alpine/edge/testing hurl
```

#### Arch Linux / Manjaro

[`hurl-bin` package] for Arch Linux and derived distros is available via [AUR].

#### NixOS / Nix

[NixOS / Nix package] is available on stable channel.

### macOS

Precompiled binaries for Intel and ARM CPUs are available at [Hurl latest GitHub release].

#### Homebrew

```shell
$ brew install hurl
```

#### MacPorts

```shell
$ sudo port install hurl
```

### FreeBSD

```shell
$ sudo pkg install hurl
```

### Windows

#### Zip File

Hurl can be installed from a standalone zip file at [Hurl latest GitHub release]. You will need to update your `PATH` variable.

#### Installer

An executable installer is also available at [Hurl latest GitHub release].

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

### conda-forge

```shell
$ conda install -c conda-forge hurl
```

Hurl can also be installed with [`conda-forge`] powered package manager like [`pixi`]. 

### Docker

```shell
$ docker pull ghcr.io/orange-opensource/hurl:latest
```

### npm

```shell
$ npm install --save-dev @orangeopensource/hurl
```

## Building From Sources

Hurl sources are available in [GitHub].

### Build on Linux

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
$ pacman -S --noconfirm pkgconf gcc glibc openssl libxml2
```

#### Alpine based distributions

```shell
$ apk add curl-dev gcc libxml2-dev musl-dev openssl-dev 
```

### Build on macOS

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
[Hurl latest GitHub release]: https://github.com/Orange-OpenSource/hurl/releases/latest
[AUR]: https://wiki.archlinux.org/index.php/Arch_User_Repository
[`hurl-bin` package]: https://aur.archlinux.org/packages/hurl-bin/
[install]: https://www.rust-lang.org/tools/install
[Rust]: https://www.rust-lang.org
[contrib on Windows section]: https://github.com/Orange-OpenSource/hurl/blob/master/contrib/windows/README.md
[NixOS / Nix package]: https://search.nixos.org/packages?from=0&size=1&sort=relevance&type=packages&query=hurl
[`conda-forge`]: https://conda-forge.org
[`pixi`]: https://prefix.dev
