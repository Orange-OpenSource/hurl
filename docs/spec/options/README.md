# Command Line Options

Hurl command line options are described with `.option` files. These files are used to generate:

- Rust code to parse this options in `hurl`/`hurlfmt` packages
- Man options


## Format all option files

```shell
$ bin/spec/options/format.py docs/spec/options/**/*.option
```

This script insures that all `.option` files are well formatted.

## Generate/Update clap command source file

```shell
$ bin/spec/options/generate_source.py docs/spec/options/hurl/*.option > packages/hurl/src/cli/options/commands.rs
$ bin/spec/options/generate_source.py docs/spec/options/hurlfmt/*.option >packages/hurlfmt/src/cli/options/commands.rs
```

This script generates Rust source file for parsing command line option.

## Generate/Update man options

```shell
$ bin/spec/options/generate_man.py docs/spec/options/hurl/*.option
$ bin/spec/options/generate_man.py docs/spec/options/hurlfmt/*.option
```

This script generates (part of) man file for `hurl` / `hurlfmt`.

A script is provided to run all these commands:

```shell
$ bin/spec/options/generate_all.py
```
