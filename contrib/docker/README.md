# Build image

```
docker build --tag hurl:latest .
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
ocker run --rm -v /tmp/test.hurl:/tmp/test.hurl -w /tmp hurl:latest --test --color /tmp/test.hurl
```

