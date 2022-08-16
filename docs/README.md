# Hurl Documentation

This directory is the canonical source for Hurl documentation. The site <https://hurl.dev>, powered by Jekyll,
is generated from it. If you want to make modifications to <https://hurl.dev>, you can make a PR
in this repo.

## Man Page

The canonical source for the Hurl man pages is at <https://github.com/Orange-OpenSource/hurl/tree/master/docs/man>.
The markdown files [`hurl.md`] and [`hurlfmt.md`] are used :

- to generate man pages [`hurl.1`] and [`hurlfmt.1`]
- to generate Markdown documentation page [`man-page.md`] for <https://hurl.dev>

```
docs/man/hurl.md => docs/man/hurl.1
docs/man/hurl.md => docs/man-page.md

docs/man/hurlfmt.md => docs/man/hurlfmt.1
```

## READMEs

[GitHub README] and [crates.io README] are generated from the canonical docs.

```
docs/*.md => README.md
docs/*.md => packages/hurl/README.md
```


[`hurl.md`]: https://github.com/Orange-OpenSource/hurl/tree/master/docs/man/hurl.md
[`hurlfmt.md`]: https://github.com/Orange-OpenSource/hurl/tree/master/docs/man/hurlfmt.md
[`hurl.1`]: https://github.com/Orange-OpenSource/hurl/tree/master/docs/man/hurl.1
[`hurlfmt.1`]: https://github.com/Orange-OpenSource/hurl/tree/master/docs/man/hurlfmt.1
[`man-page.md`]: https://github.com/Orange-OpenSource/hurl/blob/master/docs/man-page.md
[GitHub README]: https://github.com/Orange-OpenSource/hurl/blob/master/README.md
[crates.io README]: https://github.com/Orange-OpenSource/hurl/blob/master/packages/hurl/README.md

