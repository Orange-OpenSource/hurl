# Hurl Documentation

This directory is the canonical source for Hurl documentation. The site <https://hurl.dev>, powered by Jekyll,
is generated from it. If you want to modify <https://hurl.dev>, you can make a PR in this repo.

## Manual Page

The canonical source for the Hurl manual pages is at <https://github.com/Orange-OpenSource/hurl/tree/master/docs/manual>.
The markdown files [`hurl.md`] and [`hurlfmt.md`] are used :

- to generate manual pages [`hurl.1`] and [`hurlfmt.1`]
- to generate Markdown documentation page [`manual.md`] for <https://hurl.dev>

Dependencies:

```
docs/manual/hurl.md => docs/manual/hurl.1
docs/manual/hurl.md => docs/manual.md

docs/manual/hurlfmt.md => docs/manual/hurlfmt.1
```

## READMEs

[GitHub README] and [crates.io README] are generated from the canonical docs.

Dependencies:

```
docs/*.md => README.md
docs/*.md => packages/hurl/README.md
```

## Scripts

1. generate manual
2. generate <hurl.dev> manual
3. generate GitHub README
4. generate crates.io README

```bash
$ cd ..
$ python3 bin/release/gen_manpage.py docs/manual/hurl.md > docs/manual/hurl.1
$ python3 bin/release/gen_manpage.py docs/manual/hurlfmt.md > docs/manual/hurlfmt.1
$ python3 bin/docs/build_man_md.py docs/manual/hurl.md > docs/manual.md
$ python3 bin/docs/build_readme.py github > README.md
$ python3 bin/docs/build_readme.py crates > packages/hurl/README.md
```


[`hurl.md`]: https://github.com/Orange-OpenSource/hurl/tree/master/docs/manual/hurl.md
[`hurlfmt.md`]: https://github.com/Orange-OpenSource/hurl/tree/master/docs/manual/hurlfmt.md
[`hurl.1`]: https://github.com/Orange-OpenSource/hurl/tree/master/docs/manual/hurl.1
[`hurlfmt.1`]: https://github.com/Orange-OpenSource/hurl/tree/master/docs/manual/hurlfmt.1
[`manual.md`]: https://github.com/Orange-OpenSource/hurl/blob/master/docs/manual.md
[GitHub README]: https://github.com/Orange-OpenSource/hurl/blob/master/README.md
[crates.io README]: https://github.com/Orange-OpenSource/hurl/blob/master/packages/hurl/README.md

