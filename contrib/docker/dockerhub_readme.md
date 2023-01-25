# Container image has moved to github

Since hurl 2.0.0, docker hub is no more supported, please use https://github.com/Orange-OpenSource/hurl/pkgs/container/hurl :)

### Quick references

- **Maintained by**: Filipe PINTO, Fabrice REIX (Orange-OpenSource/hurl maintainers).
- **Home**: https://hurl.dev
- **Where to get help**: https://github.com/Orange-OpenSource/hurl/issues

### What is Hurl ?

![logo](https://raw.githubusercontent.com/Orange-OpenSource/hurl/master/art/logo-mini-light.svg)

Hurl is a command line tool that runs HTTP requests defined in a simple plain text format.
It can perform requests, capture values and evaluate queries on headers and body response. Hurl is very versatile: it can be used for both fetching data and testing HTTP sessions.

### How to use this image

Get Hurl version:

```
docker run --rm ghcr.io/orange-opensource/hurl:latest --version
```

Run Hurl from STDIN:

```
echo -e "GET https://hurl.dev\n\nHTTP/1.1 200" | docker run --rm -i ghcr.io/orange-opensource/hurl:latest --test --color
```

Run Hurl from FILE:

```
echo -e "GET https://hurl.dev\n\nHTTP/1.1 200" > /tmp/test.hurl
docker run --rm -v /tmp/test.hurl:/tmp/test.hurl -w /tmp ghcr.io/orange-opensource/hurl:latest --test --color /tmp/test.hurl
```
