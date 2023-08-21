# Build amd64 image

## Clone desired tag

```
tag=<desired tag, ex: 4.0.0>
organisation=<desired organisation in lowercase, ex: orange-opensource>
git clone --depth 1 https://github.com/"${organisation}"/hurl.git --branch "${tag}" /tmp/hurl-"${tag}"
```

## Prepare docker build env

```
docker rmi --force \
  ghcr.io/"${organisation}"/hurl:amd64-"${docker_build_tag}" \
  local/hurl
```

## Build

```
cd /tmp/hurl-"${tag}"
docker_build_tag=$(grep ^version packages/hurl/Cargo.toml | cut --delimiter '=' --field 2 | tr -d '" ')
echo "docker_build_tag=${docker_build_tag}"
docker_build_date=$(date "+%Y-%m-%d %H-%M-%S")
echo "docker_build_date=${docker_build_date}"
docker build --file contrib/docker/Dockerfile --build-arg docker_build_date="${docker_build_date}" --build-arg docker_build_tag="${docker_build_tag}" --tag  local/hurl --tag ghcr.io/"${organisation}"/hurl:amd64-"${docker_build_tag}" .
```

## Get docker hurl version

```
docker run --rm local/hurl --version
```

## Run docker hurl from STDIN

```
echo -e "GET https://hurl.dev\n\nHTTP 200" | docker run --rm -i local/hurl --test --color --very-verbose
```

## Run docker hurl from FILE

```
echo -e "GET https://hurl.dev\n\nHTTP 200" > /tmp/test.hurl
docker run --rm -v /tmp/test.hurl:/tmp/test.hurl local/hurl --test --color --very-verbose /tmp/test.hurl
```

## Connect to github container registry

```
echo <hurl-bot github token> | docker login ghcr.io --username hurl-bot --password-stdin
```

## Push to github registry

```
docker push ghcr.io/"${organisation}"/hurl:amd64-"${docker_build_tag}"
```

# Build arm64 image

## Clone desired tag

```
tag=<desired tag, ex: 4.0.0>
organisation=<desired organisation in lowercase, ex: orange-opensource>
git clone --depth 1 https://github.com/"${organisation}"/hurl.git --branch "${tag}" /tmp/hurl-"${tag}"
```

## Prepare docker build env

```
docker rmi --force \
  ghcr.io/"${organisation}"/hurl:arm64-"${docker_build_tag}" \
  local/hurl
```

## Build

```
cd /tmp/hurl-"${tag}"
docker_build_tag=$(grep ^version packages/hurl/Cargo.toml | cut --delimiter '=' --field 2 | tr -d '" ')
echo "docker_build_tag=${docker_build_tag}"
docker_build_date=$(date "+%Y-%m-%d %H-%M-%S")
echo "docker_build_date=${docker_build_date}"
docker build --file contrib/docker/Dockerfile --build-arg docker_build_date="${docker_build_date}" --build-arg docker_build_tag="${docker_build_tag}" --tag  local/hurl --tag ghcr.io/"${organisation}"/hurl:arm64-"${docker_build_tag}" .
```

## Get docker hurl version

```
docker run --rm local/hurl --version
```

## Run docker hurl from STDIN

```
echo -e "GET https://hurl.dev\n\nHTTP 200" | docker run --rm -i local/hurl --test --color
```

## Run docker hurl from FILE

```
echo -e "GET https://hurl.dev\n\nHTTP 200" > /tmp/test.hurl
docker run --rm -v /tmp/test.hurl:/tmp/test.hurl local/hurl --test --color /tmp/test.hurl
```

## Connect to github container registry

```
echo <hurl-bot github token> | docker login ghcr.io --username hurl-bot --password-stdin
```

## Push to github registry

```
docker push ghcr.io/"${organisation}"/hurl:arm64-"${docker_build_tag}"
```

# When all architectures images are built
## Create tag and latest manifest

```
docker manifest create \
  ghcr.io/"${organisation}"/hurl:"${docker_build_tag}" \
  --amend ghcr.io/"${organisation}"/hurl:amd64-"${docker_build_tag}" \
  --amend ghcr.io/"${organisation}"/hurl:arm64-"${docker_build_tag}"
docker manifest create \
  ghcr.io/"${organisation}"/hurl:latest" \
  --amend ghcr.io/"${organisation}"/hurl:amd64-"${docker_build_tag}" \
  --amend ghcr.io/"${organisation}"/hurl:arm64-"${docker_build_tag}"
```

## Push new manifest

```
docker manifest push ghcr.io/"${organisation}"/hurl:"${docker_build_tag}"
docker manifest push ghcr.io/"${organisation}"/hurl:latest
```
