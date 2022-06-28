# Build image

```
hurl_latest_version=$(curl --silent "https://api.github.com/repos/Orange-OpenSource/hurl/releases/latest" | jq -r .tag_name)
docker_build_date=$(date "+%Y-%m-%d %H-%M-%S")
docker build --build-arg docker_build_date="${docker_build_date}" --build-arg hurl_latest_version=${hurl_latest_version} --tag hurl:latest --tag hurl:${hurl_latest_version} .
```

# Get docker hurl version

```
docker run --rm hurl:latest --version
```

# Run docker hurl from STDIN

```
echo -e "GET https://hurl.dev\n\nHTTP/1.1 200" | docker run --rm -i hurl:latest --test --color
```

# Run docker hurl from FILE

```
echo -e "GET https://hurl.dev\n\nHTTP/1.1 200" > /tmp/test.hurl
docker run --rm -v /tmp/test.hurl:/tmp/test.hurl hurl:latest --test --color /tmp/test.hurl
```
