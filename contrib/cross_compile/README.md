# Create generic arm64 linux package

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

## Choose desired tag

```
echo -n "tag: "
read -r tag
export tag
echo "tag=${tag}"
```

## Clone desired tag

```
git clone --depth 1 https://github.com/Orange-OpenSource/hurl.git --branch "${tag}" /tmp/hurl-"${tag}"
cd /tmp/hurl-"${tag}"
```

## Run docker arm64 ubuntu

```
ubuntu_docker_image=$(grep -E "package-generic-linux-x64|runs-on" .github/workflows/package.yml | head -2 | tail -1 | cut --delimiter ":" --field 2 | tr "-" ":" | tr -d " ")
echo "ubuntu_docker_image=${ubuntu_docker_image}"
docker run --platform linux/arm64 --volume /tmp/hurl-"${tag}":/hurl --workdir /hurl -it --rm "${ubuntu_docker_image}" bash
```

## Install system prerequisistes

```
uname -m
export DEBIAN_FRONTEND=noninteractive
apt update
bin/install_prerequisites_ubuntu.sh
```

## Build

```
source bin/export_cross_compile_env.sh
bin/install_rust.sh
bin/release/release.sh
```

## Create generic arm64 linux tarball

```
export PATH="${PWD}/target/release:${PATH}"
export VERSION=$(bin/release/get_version.sh)
bin/release/man.sh
bin/release/create_tarball.sh linux
```

## Test generic arm64 linux binary

```
bin/release/install_generic_linux_package.sh
export PATH="/tmp/hurl-generic-linux:${PATH}"
bin/activate_python3_venv.sh
export PATH=/tmp/hurl-python3-venv/bin:$PATH
bin/test/test_prerequisites.sh
bin/test/test_integ.sh
```

## List generic arm64 linux package

```
exit
cd target/upload
ls -lrt hurl*
```
