#!/usr/bin/env python3
"""Build README for GitHub and crates.io.

This script uses Hurl doc to generate README suitable for GitHub and crates.io

Examples:
    $ python3 build_readme.py github > ../../README.md
    $ python3 build_readme.py crates > ../../packages/hurl/README.md

"""
import os
import re
import sys
from pathlib import Path
from textwrap import dedent

from markdown import parse_markdown, MarkdownDoc


def main(dest: str) -> int:

    header: str

    if dest == "github":
        header = dedent(
            """\
            <img src="https://raw.githubusercontent.com/Orange-OpenSource/hurl/master/art/logo-full-dark.svg?sanitize=true#gh-dark-mode-only" alt="Hurl Logo" width="264px"><img src="https://raw.githubusercontent.com/Orange-OpenSource/hurl/master/art/logo-full-light.svg?sanitize=true#gh-light-mode-only" alt="Hurl Logo" width="264px">
            
            <br/>
            
            [![deploy status](https://github.com/Orange-OpenSource/hurl/workflows/CI/badge.svg)](https://github.com/Orange-OpenSource/hurl/actions)
            [![CircleCI](https://circleci.com/gh/lepapareil/hurl/tree/master.svg?style=shield)](https://circleci.com/gh/lepapareil/hurl/tree/master)
            [![Crates.io](https://img.shields.io/crates/v/hurl.svg)](https://crates.io/crates/hurl)
            [![documentation](https://img.shields.io/badge/-documentation-informational)](https://hurl.dev)
            
            """
        )
    elif dest == "crates":
        header = dedent(
            """\
            <img src="https://raw.githubusercontent.com/Orange-OpenSource/hurl/master/art/logo-full-light.svg" alt="Hurl Logo" width="264px">
            
            <br/>
            
            [![deploy status](https://github.com/Orange-OpenSource/hurl/workflows/CI/badge.svg)](https://github.com/Orange-OpenSource/hurl/actions)
            [![CircleCI](https://circleci.com/gh/lepapareil/hurl/tree/master.svg?style=shield)](https://circleci.com/gh/lepapareil/hurl/tree/master)
            [![Crates.io](https://img.shields.io/crates/v/hurl.svg)](https://crates.io/crates/hurl)
            [![documentation](https://img.shields.io/badge/-documentation-informational)](https://hurl.dev)
            
            """
        )
    else:
        sys.stderr.write("build_readme.py [github, crates]\n")
        return os.EX_USAGE

    header_md = parse_markdown(text=header)

    home = Path("../../docs/home.md").read_text()
    # We adapt the "Why Hurl" part to transform h2 tag back to markdown

    def showcase_rep(m):
        return f"<li><b>{m.group(1)}:</b> {m.group(2).lower()}</li>"

    home = re.sub(
        r"""<li class="showcase-item"><h2 class="showcase-item-title">(.+)</h2>(.+)</li>""",
        showcase_rep,
        home,
    )
    home_md = parse_markdown(text=home)
    # Remove canonical logo, will be replaced with GitHub flavored logo
    logo_nodes = [
        home_md.children[0],
        home_md.children[1],
    ]
    home_md.remove_nodes(logo_nodes)

    samples_md = parse_markdown(text=Path("../../docs/samples.md").read_text())
    usage_md = parse_markdown(text=Path("../../docs/man-page.md").read_text())

    installation_md = parse_markdown(
        text=Path("../../docs/installation.md").read_text()
    )

    body_md = MarkdownDoc()
    body_md.extend(samples_md)
    body_md.extend(usage_md)
    body_md.extend(installation_md)
    toc = body_md.toc()
    toc_md = parse_markdown(text=toc)

    readme_md = MarkdownDoc()
    readme_md.extend(header_md)

    readme_md.extend(home_md)
    readme_md.extend(toc_md)
    readme_md.extend(body_md)

    readme = readme_md.to_text()

    # Replace canonical links to hurl.dev links
    readme = re.sub(
        r"/docs/(.*)\.md",
        r"https://hurl.dev/docs/\1.html",
        readme,
    )
    readme = readme.replace("blog.md", "https://hurl.dev/blog/")

    print(readme)
    return os.EX_OK


if __name__ == "__main__":
    main(dest=sys.argv[1])
