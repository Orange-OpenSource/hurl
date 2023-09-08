# Build amd64 generic binary

## Prepare docker build env

```
docker system prune -fa
docker buildx prune -fa
docker buildx rm mybuilder
export DOCKER_CLI_EXPERIMENTAL=enabled
sudo apt-get install -y qemu-user-static
ls -l /usr/bin/qemu-aarch64-static
qemu-aarch64-static --version
sudo apt-get install -y binfmt-support
update-binfmts --version
docker run --rm --privileged multiarch/qemu-user-static --reset -p yes
docker buildx create --name mybuilder
docker buildx use mybuilder
docker buildx inspect --bootstrap
docker buildx ls
```

## Clone desired tag

```
tag=<desired tag, ex: 4.0.0>
echo "tag=${tag}"
git clone --depth 1 https://github.com/Orange-OpenSource/hurl.git --branch "${tag}" /tmp/hurl-"${tag}"
cd /tmp/hurl-"${tag}"
```

## Run docker arm64 ubuntu

```
ubuntu_docker_image=$(grep -E "package-generic-linux-x64|runs-on" .github/workflows/package.yml | head -2 | tail -1 | cut --delimiter ":" --field 2 | tr "-" ":" | tr -d " ")
echo "ubuntu_docker_image=${ubuntu_docker_image}"
docker run --platform linux/arm64 --volume /tmp/hurl-"${tag}":/hurl --workdir /hurl -it --rm "${ubuntu_docker_image}" bash
```

## Build

```
uname -m
export DEBIAN_FRONTEND=noninteractive
apt update
apt install -y sudo curl g++-aarch64-linux-gnu libc6-dev-arm64-cross libxml2-dev librust-openssl-dev pkg-config
bin/install_prerequisites_ubuntu.sh
bin/install_rust.sh
export PKG_CONFIG_ALLOW_CROSS=1
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
export CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc
export CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++
bin/release/release.sh
```

## Create generic linux arm64 package

```
export PATH="${PWD}/target/release:${PATH}"
export VERSION=$(bin/release/get_version.sh)
bin/release/man.sh
bin/release/create_tarball.sh linux
```

## Test generic linux arm64 package

```
bin/release/install_generic_linux_package.sh
export PATH="/tmp/hurl-generic-linux:${PATH}"
bin/install_python3_venv.sh
export PATH=/tmp/hurl-python3-venv/bin:$PATH
bin/test/test_prerequisites.sh
bin/test/test_integ.sh
```

## List package

```
exit
cd target/upload
ls -lrt hurl*
```
