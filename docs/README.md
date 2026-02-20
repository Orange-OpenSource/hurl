# Hurl Documentation

This directory is the canonical source for Hurl documentation. The site <https://hurl.dev>, powered by Jekyll,
is generated from it. If you want to modify <https://hurl.dev>, you can make a PR in this repo.

> [!TIP]
> TLDR
> To update all docs:
> ```shell
> $ bin/docs/update_all.sh
> ```

## Documentation Generation

Some files are dependent and needs to be generated appropriated.
- `docs/spec/options/**/*.option`: define the command line option of `hurl` and `hurlfmt`. These are text declarative
files that will update project files (Rust files to produce the output of `--help`, Rust file options etc..., shell completion
script etc...). These files are also used to generate part of man files `docs/manual/hurl.md`, `docs/manual/hurlfmt.md`
- `docs/manual/hurl.md`/`docs/manual/hurlfmt.md`: Markdown source files of man pages `hurl.1``hurlfmt.1`
- `README.md` / `packages/hurl/README.md`, / `packages/hurlfmt/README.md`: GitHub and <https://crates.io> READMEs. These
files are entirely generated from previous files (`.options` files, `.md` manual) 


### Options

Hurl options files describes each option of `hurl` and `hurlfmt`. They're used to generated various files of the project. 


### Manual Page

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

### READMEs

[GitHub README] and [crates.io README] are generated from the canonical docs.

Dependencies:

```
docs/*.md => README.md
docs/*.md => packages/hurl/README.md
```

### Scripts to update manual and READMEs

1. generate manual
2. generate <hurl.dev> manual
3. generate GitHub README
4. generate crates.io README

```bash
$ cd ..
$ python3 bin/docs/build_man.py docs/manual/hurl.md > docs/manual/hurl.1
$ python3 bin/docs/build_man.py docs/manual/hurlfmt.md > docs/manual/hurlfmt.1
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

