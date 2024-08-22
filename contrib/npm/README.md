# Hurl on npm

Hurl is [distributed on npm] as a thin wrapper around the native binary.

## Build

## Publish

To publish a new version of the package `PACKAGE_VERSION` using the binary `HURL_VERSION`:

```shell
$ cp docs/manual/*.1 contrib/npm/hurl/docs/
$ python3 ./contrib/npm/check_publish.py $HURL_VERSION $PACKAGE_VERSION
$ rm -rfd contrib/npm/hurl/dist contrib/npm/hurl/node_modules
$ npm publish --dry-run contrib/npm/hurl/
$ npm publish contrib/npm/hurl/
```


[distributed on npm]: https://www.npmjs.com/package/@orangeopensource/hurl