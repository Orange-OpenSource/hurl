# Hurl on Homebrew

## Publish

<https://docs.brew.sh/How-To-Open-a-Homebrew-Pull-Request>

On local terminal, to publish a new version `x.y.z`:

```shell
$ brew bump-formula-pr --url https://github.com/Orange-OpenSource/hurl/archive/refs/tags/x.y.z.tar.gz hurl --verbose --dry-run
$ brew bump-formula-pr --url https://github.com/Orange-OpenSource/hurl/archive/refs/tags/x.y.z.tar.gz hurl --verbose
```

Git config is from `/opt/homebrew`.