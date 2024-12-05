# Build and push to PPA

## Create the PPA for Ubuntu focal and newer

- Go to https://launchpad.net
- Create an account
- Create PPA
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
mkdir -p /tmp/gpg
chmod 777 /tmp/gpg
gpg_keyid=$(gpg --list-keys | grep -E "^ " | tr -d ' ')
gpg --armor --export "${gpg_keyid}" > /tmp/gpg/mypublickey.asc
gpg --armor --export-secret-keys "${gpg_keyid}" > /tmp/gpg/myprivatekey.asc

```

## Choose Hurl version and Ubuntu codename

```
echo -n "hurl_version=" && read -r hurl_version
echo -n "Ubuntu codename=" && read -r codename

```

## Run ubuntu container

```
docker run -it --rm --env gpg_keyid="${gpg_keyid}" --env hurl_version="${hurl_version}" --env codename="${codename}" --env date="$(date -u "+%a, %d %b %Y %H:%M:%S")" --volume "/tmp/gpg:/tmp" ubuntu:"${codename}" bash

```

## Install user prerequisites and build dependencies

```
export DEBIAN_FRONTEND=noninteractive
apt update && \
    apt install -y gpg git curl wget vim xz-utils gettext moreutils && \
    apt install -y pkg-config gcc curl libxml2-dev libssl-dev devscripts debhelper

```

## Import GPG key into container

```
export GPG_TTY=$(tty)
gpg --import /tmp/mypublickey.asc
gpg --import /tmp/myprivatekey.asc

```

## Clone Hurl

```
rm -fr /tmp/ppa || true
git clone --depth 1 https://github.com/Orange-OpenSource/hurl.git /tmp/ppa/hurl-ppa
git clone --depth 1 https://github.com/Orange-OpenSource/hurl.git --branch "${hurl_version}" /tmp/ppa/hurl-"${hurl_version}"
cd /tmp/ppa/hurl-"${hurl_version}"
cp -r ../hurl-ppa/contrib/ppa/debian .

```

## Minimize repo

```
rm -fr .circleci \
       .github \
       .git \
       rustfmt.toml \
       art \
       contrib \
       integration \
       RELEASING.md \
       README.md \
       CONTRIBUTING.md
while read -r dir ; do
    rm -fr $dir
done < <(find bin -mindepth 1 -type d | grep -v "bin/release")
while read -r file ; do
    rm -fr $file
done < <(find bin -type f | grep -Ev "man\.sh|release\.sh|gen_manpage\.py")
while read -r dir ; do
    rm -fr $dir
done < <(find docs -mindepth 1 -type d | grep -v "docs/manual")
while read -r file ; do
    rm -fr $file
done < <(find docs -type f | grep -Ev "manual/")

```

## Minimize offline rust and cargo installer

```
rust_version=$(grep '^rust-version' packages/hurl/Cargo.toml | cut -f2 -d'"')
for arch in x86_64 aarch64 ; do
    package="rust-${rust_version}-${arch}-unknown-linux-gnu"
    packagelight="${package}-light"
    wget "https://static.rust-lang.org/dist/${package}.tar.xz"
    xz -T0 -vd "${package}.tar.xz"
    tar -xf "${package}.tar"
    dirs_to_delete=$(find "${package}" -type d | cut --delimiter "/" --field 1,2 | grep "/" | grep -Ev  "/cargo$|/rust-std-${arch}-unknown-linux-gnu$|/rustc$" | sort -u | tr '\n' ' ')
    rm -fr $dirs_to_delete
    grep -E "^cargo$|^rust-std-${arch}-unknown-linux-gnu$|^rustc$" "${package}/components" | sponge "${package}/components"
    mv "${package}" "${packagelight}"
    tar cf "${packagelight}.tar" "${packagelight}"
    xz -T0 -9 -v "${packagelight}.tar"
    rm -fr "${package}.tar" "${packagelight}"
done
```

## Install rust and cargo

```
arch=$(uname -m)
package="rust-${rust_version}-${arch}-unknown-linux-gnu"
packagelight="${package}-light"
tar xf "${packagelight}".tar.xz
./"${packagelight}"/install.sh --destdir=/tmp/rust --disable-ldconfig
export PATH="/tmp/rust/usr/local/bin:$PATH"
rustc --version
cargo --version
rm -fr "${package}"

```

## Create vendor.tar.xz (offline cargo deps)

```
cargo vendor
tar pcfJ vendor.tar.xz vendor
rm -rf vendor

```

## Create debian/cargo-checksum.json

```
cargo package --package hurl
sum=$(sha256sum target/package/hurl-"${hurl_version}".crate | cut -d' ' -f1 | tr -d ' ')
echo "{\"package\": \"${sum}\",\"files\": {}}" > debian/cargo-checksum.json

```

## Create debian/cargo_home/config and update .cargo/config

```
mkdir -p debian/cargo_home
cp Cargo.toml debian/cargo_home/config
{
cat .cargo/config.toml
echo
cat debian/cargo.config
} > .cargo/config

```

## Create debian/changelog

```
envsubst < debian/changelog.template > debian/changelog
cat debian/changelog

```

## Create deb package source

```
yes | debuild -S -sa -k"${gpg_keyid}"

```

## Verify deb package source

```
cd ..
ls -l hurl_*

```

## Push to PPA

```
dput ppa:lepapareil/hurl hurl_*_source.changes

```

## Clean debuild packages

```
rm -fr hurl_"${hurl_version}"*

```

# Test Hurl from published PPA

## Choose Hurl version and Ubuntu codename

```shell
echo -n "hurl_version=" && read -r hurl_version
echo -n "Ubuntu codename=" && read -r codename
docker run -it --rm --env hurl_version="${hurl_version}" --env codename="${codename}" --volume "/tmp:/tmp" "ubuntu:${codename}" bash

```

## Install Hurl
```
export DEBIAN_FRONTEND=noninteractive
yes | unminimize
# apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 1550DC447B95F03B
apt install -y git sudo man-db curl software-properties-common && \
    apt-add-repository -y ppa:lepapareil/hurl && \
    apt list --all-versions hurl && \
    apt install -y hurl="${hurl_version}"*
```


## Check Hurl installed version

```
hurl --version
hurlfmt --version
man hurl | head -1
man hurlfmt | head -1
```

## Run Hurl from STDIN

```
echo -e "GET https://hurl.dev\n\nHTTP 200" | hurl --test --color
```

## Run  Hurl from FILE

```
echo -e "GET https://hurl.dev\n\nHTTP 200" > /tmp/test.hurl
hurl --test --color /tmp/test.hurl
```
