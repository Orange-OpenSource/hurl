# Connect to github container registry

```
echo <hurl-bot github token> | docker login ghcr.io --username hurl-bot --password-stdin
```
# Clone desired tag

```
git clone --depth 1 https://github.com/Orange-OpenSource/hurl.git --branch <desired tag> /tmp/hurl
```

# Build image

```
cd /tmp/hurl
tag=$(git rev-parse --abbrev-ref HEAD | tr '/' '-')
docker_build_date=$(date "+%Y-%m-%d %H-%M-%S")
docker builder prune --all
docker build . --file contrib/docker/Dockerfile --build-arg docker_build_date="${docker_build_date}" --build-arg hurl_branch=${tag} --tag ghcr.io/orange-opensource/hurl:latest --tag ghcr.io/orange-opensource/hurl:${tag}
```

# Get docker hurl version

```
docker run --rm ghcr.io/orange-opensource/hurl:latest --version
```

# Run docker hurl from STDIN

```
echo -e "GET https://hurl.dev\n\nHTTP 200" | docker run --rm ghcr.io/orange-opensource/hurl:latest --test --color
```

# Run docker hurl from FILE

```
echo -e "GET https://hurl.dev\n\nHTTP 200" > /tmp/test.hurl
docker run --rm -v /tmp/test.hurl:/tmp/test.hurl ghcr.io/orange-opensource/hurl:latest --test --color /tmp/test.hurl
```

# Push to github container registry

```
docker push ghcr.io/orange-opensource/hurl:${tag}
docker push ghcr.io/orange-opensource/hurl:latest
```
