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
hurl_version=<hurl tag>
```
## Run ubuntu container

```
docker run -it --rm --env GPG_KEYID=${gpg_keyid} --env HURL_VERSION=${hurl_version} --volume /tmp:/tmp ubuntu:24.04 bash
```

## Install user prerequisites

```
export DEBIAN_FRONTEND=noninteractive
apt update
apt install -y \
    gpg \
    git \
    curl \
    vim \
    xz-utils
```

## Install build dependencies

```
apt install -y \
    pkg-config build-essential curl libxml2-dev libssl-dev \
    devscripts debhelper dh-cargo
```

## Import GPG key into container

```
gpg --import /tmp/mypublickey.asc
gpg --import /tmp/myprivatekey.asc
```

## Clone hurl

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
rm /usr/bin/rustc || true
rm /usr/bin/cargo || true
ln -s /root/.cargo/bin/rustc /usr/bin/rustc
ln -s /root/.cargo/bin/cargo /usr/bin/cargo
rustc --version
cargo --version
```

## Create vendor.tar.xz (offline cargo deps)

```
cargo vendor
tar pcfJv vendor.tar.xz vendor
rm -rf vendor
```

## Create debian/cargo-checksum.json

```
cargo package --package hurl
sum=$(sha256sum target/package/hurl-"${HURL_VERSION}".crate | cut -d' ' -f1 | tr -d ' ')
echo "{\"package\": \"${sum}\",\"files\": {}}" > debian/cargo-checksum.json
```

## Create debian/cargo_home/config

```
mkdir -p debian/cargo_home
cp Cargo.toml debian/cargo_home/config
```

## Create .cargo/config

```
{
cat .cargo/config.toml
echo
cat debian/cargo.config
} > .cargo/config
```

## Create deb package source

```
debuild -S -sa -k"${GPG_KEYID}"
cd ..
ls -l hurl_*
```

## Push to PPA

```
dput ppa:<USER_NAME>/<PPA_NAME> hurl_*_source.changes
```

## Install Hurl from PPA

```shell
apt update
apt install -y software-properties-common
add-apt-repository -y ppa:<USER_NAME>/<PPA_NAME>
apt install -y hurl
```
