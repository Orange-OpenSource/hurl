# Support for Hurl Syntax Highlighting

This enables syntax coloring for Hurl files in [Sublime Text], and in any tool
that reuses Sublime Text syntax definitions, such as [bat].

The `Hurl.sublime-syntax` definition covers methods, URLs, versions and status,
sections, headers and parameters, queries, predicates, filters, functions,
templates/placeholders (`{{ ... }}`), comments, and bodies (JSON, XML,
multiline strings, oneline strings, and `base64,`/`hex,`/`file,` values).

## Sublime Text

File-extension detection is built into the syntax (`.hurl`), so a single file
is all that is needed.

1. In Sublime Text, open `Preferences > Browse Packages…` to open the
   `Packages` directory.
2. Copy `Hurl.sublime-syntax` into the `User` sub-directory:

   ```bash
   # Linux
   cp Hurl.sublime-syntax ~/.config/sublime-text/Packages/User/

   # macOS
   cp Hurl.sublime-syntax "~/Library/Application Support/Sublime Text/Packages/User/"

   # Windows
   copy Hurl.sublime-syntax "%APPDATA%\Sublime Text\Packages\User\"
   ```

3. Open any `.hurl` file; the `Hurl` syntax is applied automatically.

## bat

[bat] reuses Sublime Text syntaxes.

```bash
mkdir -p "$(bat --config-dir)/syntaxes"
cp Hurl.sublime-syntax "$(bat --config-dir)/syntaxes/"
bat cache --build
```

Then:

```bash
bat test.hurl
```

[Sublime Text]: https://www.sublimetext.com
[bat]: https://github.com/sharkdp/bat
