# Quick references

- **Maintained by**: Filipe PINTO, Fabrice REIX (Orange-OpenSource/hurl maintainers).
- **Home**: https://hurl.dev
- **Where to get help**: https://github.com/Orange-OpenSource/hurl/issues

# What is Hurl ?

![logo](https://raw.githubusercontent.com/Orange-OpenSource/hurl/master/art/logo-mini-light.svg)

Hurl is a command line tool that runs HTTP requests defined in a simple plain text format.
It can perform requests, capture values and evaluate queries on headers and body response. Hurl is very versatile: it can be used for both fetching data and testing HTTP sessions.

# How to use this image

Get Hurl version:

```
docker run --rm orangeopensource/hurl:latest --version
```

Run Hurl from STDIN:

```
echo -e "GET https://hurl.dev\n\nHTTP/1.1 200" | docker run --rm -i orangeopensource/hurl:latest --test --color
```

Run Hurl from FILE:

```
echo -e "GET https://hurl.dev\n\nHTTP/1.1 200" > /tmp/test.hurl
docker run --rm -v /tmp/test.hurl:/tmp/test.hurl -w /tmp orangeopensource/hurl:latest --test --color /tmp/test.hurl
```

