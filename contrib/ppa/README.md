## Create the PPA

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
gpg_keyid=$(gpg --list-keys | grep -E "^ " | tr -d ' ')
gpg --armor --export "${gpg_keyid}" > /tmp/mypublickey.asc
gpg --armor --export-secret-keys "${gpg_keyid}" > /tmp/myprivatekey.asc
```
## Choose Hurl version

```
hurl_version=<hurl tag>
```
## Run ubuntu focal container

```
docker run -it --rm --env GPG_KEYID=${gpg_keyid} --env HURL_VERSION=${hurl_version} --volume /tmp:/tmp ubuntu:20.04 bash
```

## Install user prerequisites

```
export DEBIAN_FRONTEND=noninteractive
apt update
apt install -y \
    gpg \
    git \
    curl \
    wget \
    vim \
    xz-utils
```

## Install build dependencies

```
apt install -y \
    pkg-config gcc curl libxml2-dev libssl-dev \
    devscripts debhelper
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
cp -r contrib/ppa/debian .
```

## Install rust and cargo

```
rust_version=$(grep '^rust-version' packages/hurl/Cargo.toml | cut -f2 -d'"')
for arch in x86_64 aarch64 ; do 
    wget "https://static.rust-lang.org/dist/rust-${rust_version}-${arch}-unknown-linux-gnu.tar.gz"
done
rust_architecture=$(uname -m)
package="rust-${rust_version}-${rust_architecture}-unknown-linux-gnu"
tar xfv "${package}.tar.gz"
./"${package}"/install.sh --destdir=/tmp/rust --disable-ldconfig
export PATH="/tmp/rust/usr/local/bin:$PATH"
rustc --version
cargo --version
rm -fr rust-"${rust_version}"-x86_64-unknown-linux-gnu
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
dput ppa:lepapareil/hurl hurl_*_source.changes
```

## Copy published PPA (focal) to newers ubuntu codenames

![image](https://github.com/user-attachments/assets/8e7d506a-d266-44eb-8d2f-48431defb890)

![image](https://github.com/user-attachments/assets/9304a5d9-1422-4320-915f-11b3cf3d1c27)

## Install and test Hurl from PPA

```shell
codename=<ubuntu codename from focal to latest>
hurl_version=<hurl tag>
docker run -it --rm --env GPG_KEYID=${gpg_keyid} --env HURL_VERSION=${hurl_version} --env CODENAME=${codename} --volume /tmp:/tmp ubuntu:${codename} bash
export DEBIAN_FRONTEND=noninteractive
apt update
apt install -y git sudo curl software-properties-common
# apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 1550DC447B95F03B
apt-add-repository -y ppa:lepapareil/hurl
apt install -y hurl
hurl --version
hurlfmt --version
git clone --depth 1 https://github.com/Orange-OpenSource/hurl.git --branch "${HURL_VERSION}" /tmp/hurl-"${HURL_VERSION}"
cd /tmp/hurl-"${HURL_VERSION}"
./bin/install_prerequisites_ubuntu.sh
./bin/install_python3_venv.sh
./bin/test/test_prerequisites.sh
./bin/test/test_integ.sh
```
