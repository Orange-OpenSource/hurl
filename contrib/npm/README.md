# Hurl on npm

Hurl is [distributed on npm] as a thin wrapper around the native binary.

## Build

## Publish

To publish a new version `x.y.z`:

```
$ cp docs/manual/*.1 contrib/npm/hurl/docs/
$ python3 ./contrib/npm/check_publish.py x.y.z
$ npm publish --dry-run contrib/npm/hurl/
$ npm publish contrib/npm/hurl/
```


[distributed on npm]: https://www.npmjs.com/package/@orangeopensource/hurl