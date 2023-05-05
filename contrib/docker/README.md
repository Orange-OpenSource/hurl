# Connect to github container registry

```
echo <hurl-bot github token> | docker login ghcr.io --username hurl-bot --password-stdin
```

# Build image

```
hurl_latest_version=$(curl --silent "https://api.github.com/repos/Orange-OpenSource/hurl/releases/latest" | jq -r .tag_name)
docker_build_date=$(date "+%Y-%m-%d %H-%M-%S")
docker builder prune --all
docker build --build-arg docker_build_date="${docker_build_date}" --build-arg hurl_latest_version=${hurl_latest_version} --tag ghcr.io/orange-opensource/hurl:latest --tag ghcr.io/orange-opensource/hurl:${hurl_latest_version} .
```

# Get docker hurl version

```
docker run --rm ghcr.io/orange-opensource/hurl:latest --version
```

# Run docker hurl from STDIN

```
echo -e "GET https://hurl.dev\n\nHTTP 200" | docker run --rm -i ghcr.io/orange-opensource/hurl:latest --test --color
```

# Run docker hurl from FILE

```
echo -e "GET https://hurl.dev\n\nHTTP 200" > /tmp/test.hurl
docker run --rm -v /tmp/test.hurl:/tmp/test.hurl ghcr.io/orange-opensource/hurl:latest --test --color /tmp/test.hurl
```

# Push to github container registry

```
docker push ghcr.io/orange-opensource/hurl:${hurl_latest_version}
docker push ghcr.io/orange-opensource/hurl:latest
```
