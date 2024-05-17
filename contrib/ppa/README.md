# WIP !!

## Create the PPA

- Go to https://launchpad.net
- Create an account
- Create you PPA
- Add your GPG Public key to https://keyserver.ubuntu.com/#submitKey
- Add your GPG public key to your PPA

```
########## gpg memo ##########

# get <gpg_keyid>
gpg_keyid=$(gpg --list-keys | grep -E "^ " | tr -d ' ')

# get <GPG-FINGERPRINT>
gpg --fingerprint "${gpg_keyid}"

# export ascii-armored gpg public key
gpg --armor --export "${gpg_keyid}"

# export public and private key
gpg --armor --export "${gpg_keyid}" > /tmp/mypublickey.asc
gpg --armor --export-secret-keys "${gpg_keyid}" > /tmp/myprivatekey.asc

# import public and private key
gpg --import /tmp/mypublickey.asc
gpg --import /tmp/myprivatekey.asc
```

## Export gpg key

```
gpg_keyid=$(gpg --list-keys | grep -E "^ " | tr -d ' ')
gpg --armor --export "${gpg_keyid}" > /tmp/mypublickey.asc
gpg --armor --export-secret-keys "${gpg_keyid}" > /tmp/myprivatekey.asc
```
## Choose Hurl version

```
hurl_version=4.3.0
```
## Run ubuntu container

```
docker run -it --rm --env GPG_KEYID=${gpg_keyid} --env HURL_VERSION=${hurl_version} --volume /tmp:/tmp ubuntu:22.04 bash
```

## Install user prerequisites

```
export DEBIAN_FRONTEND=noninteractive
apt update
apt install -y \
    git \
    curl \
    vim \
    xz-utils
```

## Clone hurl 4.3.0

```
rm -fr /tmp/ppa || true
git clone --depth 1 https://github.com/Orange-OpenSource/hurl.git --branch "${HURL_VERSION}" /tmp/ppa/hurl-"${HURL_VERSION}"
cd /tmp/ppa/hurl-"${HURL_VERSION}"
```

## Install rust and cargo

```
rust_version=$(grep '^rust-version' packages/hurl/Cargo.toml | cut -f2 -d'"')
curl https://sh.rustup.rs -sSfkL | sh -s -- -y --default-toolchain "${rust_version}"
. "$HOME/.cargo/env"
rm /usr/bin/rustc
rm /usr/bin/cargo
ln -s /root/.cargo/bin/rustc /usr/bin/rustc
ln -s /root/.cargo/bin/cargo /usr/bin/cargo
rustc --version
cargo --version
```

## Install build dependencies

```
apt install -y \
    pkg-config build-essential curl libxml2-dev libssl-dev \
    devscripts debhelper dh-cargo
```

## Create debian dir tree

```
mkdir debian debian/source debian/cargo_home
```

## Create debian/source/format file

```
echo "3.0 (native)" > debian/source/format
```

## Create debian/compat file

```
echo "10" > debian/compat
```

## Create debian/copyright file

```
cat << EOF > debian/copyright
Format: https://www.debian.org/doc/packaging-manuals/copyright-format/1.0/
Upstream-Name: hurl
Upstream-Contact: https://github.com/Orange-OpenSource/hurl/issues
Source: https://github.com/Orange-OpenSource/hurl

Files: *
Copyright: 2024 Jean Christophe AMIEL, Fabrice REIX, Filipe PINTO
License: Apache-2.0

License: Apache-2.0
 License detail can be found at "https://github.com/Orange-OpenSource/hurl/blob/master/LICENSE".

EOF
```

## Create debian/control file

```
cat << EOF > debian/control
Source: hurl
Section: utils
Priority: optional
Maintainer: lepapareil <filipe.pinto@orange.com>
Build-Depends:  debhelper (>= 10), dh-cargo, rustc, cargo, curl, libxml2-dev, libssl-dev
Standards-Version: 4.6.0
Homepage: https://hurl.dev
Rules-Requires-Root: no

Package: hurl
Architecture: any
Depends: \${shlibs:Depends}, \${misc:Depends}
Description: Hurl is a command line tool that runs HTTP requests defined in a simple plain text format.
 It can chain requests, capture values and evaluate queries on headers and body response. Hurl is very versatile: it can be used for both fetching data and testing HTTP sessions.
 Hurl makes it easy to work with HTML content, REST / SOAP / GraphQL APIs, or any other XML / JSON based APIs.
EOF
```

## Create debian/changelog file

```
cat << EOF > debian/changelog
hurl (${HURL_VERSION}) bionic; urgency=medium

  * Initial Release.

 -- lepapareil <filipe.pinto@orange.com>  Fri, 17 May 2024 13:30:36 +0200

hurl (4.3.0) focal; urgency=medium

  * Initial Release.

 -- lepapareil <filipe.pinto@orange.com>  Fri, 17 May 2024 13:30:36 +0200

hurl (4.3.0) jammy; urgency=medium

  * Initial Release.

 -- lepapareil <filipe.pinto@orange.com>  Fri, 17 May 2024 13:30:36 +0200
EOF
```

## create debian/rules file

```
cat << EOF > debian/rules
#!/usr/bin/make -f

%:
	dh \$@ --buildsystem=cargo

override_dh_auto_build:
	cargo build --release --package hurl

EOF
chmod u+x debian/rules
```

## Create debian/cargo_home/config

```
ln -s Cargo.toml debian/cargo_home/config
```

## Create debian/cargo-checksum.json

```
cargo package --package hurl
sum=$(sha256sum target/package/hurl-"${HURL_VERSION}".crate | cut -d' ' -f1 | tr -d ' ')
echo "{\"package\": \"${sum}\",\"files\": {}}" > debian/cargo-checksum.json
```

## Import GPG key into container

```
gpg --import /tmp/mypublickey.asc
gpg --import /tmp/myprivatekey.asc
```

## Create deb package source

### Only source

```
debuild -S -sa -k"${GPG_KEYID}"
```

### Source and build

```
debuild -k"${GPG_KEYID}"
```
## List deb package source files

```
cd ..
ls -l hurl_*
```

## Push to PPA

```
dput ppa:<USER_NAME>/<PPA_NAME> hurl_*_source.changes
```
