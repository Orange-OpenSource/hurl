#!/usr/bin/env python3
"""
Build a standalone Markdown file of all the documentation. All links and anchors are rewritten so the
links are functional: during the concatenation of two files, the script insures that an anchor is well
specific to a given pages. "The essential, it works": means that while this script is working, it may be
not easy to maintain it.

Examples:
    $ python3 bin/docs/build_standalone_md.py > docs/standalone/hurl-5.0.1.md
"""

import os
import re
import sys
import unicodedata
from datetime import datetime
from pathlib import Path

import markdown
import tomllib
from markdown import Header, MarkdownDoc, Paragraph, RefLink, Table, Whitespace


def add_section_header(doc: MarkdownDoc, title: str):
    """Add a section header h1 to a Markdown document, with a given title"""
    node = Header(title=title, level=1)
    add_header_id(header=node, prefix=None)
    doc.add_child(node)
    node = Whitespace(content="\n")
    doc.add_child(node)


def add_sections(doc: MarkdownDoc, title: str | None, files: [str]):
    """Add a new section to a markdown documentation, using a list of files to concatenate"""
    if title:
        add_section_header(doc=doc, title=title)

    for file in files:
        sys.stderr.write(f">>> Processing <{file}>...\n")
        path = Path(file)
        text = path.read_text()
        file_md = markdown.parse_markdown(text=text)
        file_md.indent()

        # All ref links (https://daringfireball.net/projects/markdown/syntax) are inlined so we can concatenate
        # multiple documents without any problem
        #
        # Before:
        # ```markdown
        # Some bla bal [a reference][ref]
        # [ref]: https://foo.com
        # ```
        #
        # After:
        # ```markdown
        # Some bla bal [a reference](https://foo.com)
        # ```
        inline_ref_link(md=file_md)

        # Anchors are normalize so we can concatenate multiple documents that have the same anchors
        #
        # Before:
        # ```markdown
        # Some bla bal [a reference](#anchor)
        # ```
        #
        # After:
        # ```markdown
        # Some bla bal [a reference](#name-of-the-document-anchor)

        anchors_prefix = f"{title} {path.stem}"
        anchors_prefix = slugify(anchors_prefix)
        rewrite_links(md=file_md, prefix=anchors_prefix)

        hr = Paragraph(content="\n\n<hr>\n\n")
        file_md.add_child(hr)

        doc.extend(file_md)


def add_header_id(header: Header, prefix: str | None):
    """Add an anchor id to a header
    Example: `# Some title` => `# Some title {#a-prefix-some-title}`
    """
    slug = slugify(header.title)
    if prefix:
        _id = f"{prefix}-{slug}"
    else:
        _id = slug
    header.id = _id
    header.update_content()


def slugify(text: str) -> str:
    """Makes a slug from a text."""
    text = unicodedata.normalize("NFKD", text).encode("ascii", "ignore").decode("ascii")
    text = re.sub(r"[^\w\s/-]", "", text).strip().lower()
    return re.sub(r"[-\s]+", "-", text).replace("/", "")


def section_from_page(page: str):
    """Returns the section title from a page ex: "manual.md" => "Getting Started" """
    if page in ["home.md"]:
        return "Introduction"
    elif page in ["license.md"]:
        return "Resources"
    elif page in [
        "installation.md",
        "manual.md",
        "sample.md",
        "running-tests.md",
        "frequently-asked-questions.md",
    ]:
        return "Getting Started"
    else:
        return "File Format"


def rewrite_links(md: MarkdownDoc, prefix: str):
    """When multiple Markdown documents are concatenate, we need to rewrite links and anchor because
    some anchors can overlapped and documents are merged into a single document."""
    # Find all headers and add an id specific to the page
    # `# Some title` => `# Some title {#some-title}`
    headers = [c for c in md.children if isinstance(c, Header)]
    for header in headers:
        add_header_id(header, prefix=prefix)

    # Replace `[Foo](#anchor)` => `[Foo](#current-page-anchor)`
    nodes = [c for c in md.children if isinstance(c, Paragraph) or isinstance(c, Table)]
    for node in nodes:

        def repl(match_obj):
            title = match_obj.group("title")
            anchor = match_obj.group("anchor")
            _id = f"#{prefix}-{anchor}"
            return f"[{title}]({_id})"

        node.content = re.sub(
            r"\[(?P<title>.+?)]\(#(?P<anchor>.+?)\)", repl, node.content
        )

    # Replace `[Foo](/docs/some-page.md#anchor)` => `[Foo](#some-page-anchor)`
    nodes = [c for c in md.children if isinstance(c, Paragraph) or isinstance(c, Table)]
    for node in nodes:

        def repl(match_obj):
            old = match_obj.group(0)
            title = match_obj.group("title")
            page = match_obj.group("page")
            section = section_from_page(page)
            section = slugify(section)
            page = page[:-3]  # Remove .md extension
            anchor = match_obj.group("anchor")
            if anchor:
                _id = f"#{section}-{page}-{anchor}"
            else:
                _id = f"#{section}-{page}"
            new = f"[{title}]({_id})"
            sys.stderr.write(f"Replace `{old}` to `{new}\n")
            return new

        node.content = re.sub(
            r"\[(?P<title>.+?)]\(/docs/(?P<page>[a-zA-Z0-9-/]+?\.md)#?(?P<anchor>[a-z0-9-]+?)?\)",
            repl,
            node.content,
        )

    # Replace Manual links
    # `<a href="#aws-sigv4" id="aws-sigv4">`
    tables = [c for c in md.children if isinstance(c, Table)]
    for table in tables:

        def repl(match_obj):
            href = match_obj.group("href")
            _id = match_obj.group("_id")
            if href != _id:
                return f'<a href="{href}" id="{_id}">'
            else:
                return f'<a href="#{prefix}-{href}" id="{prefix}-{_id}">'

        table.content = re.sub(
            r"<a href=\"#(?P<href>.+?)\" id=\"(?P<_id>.+?)\">", repl, table.content
        )
        table.reformat()


def inline_ref_link(md: MarkdownDoc):
    """Ref links are inline: as documents are merged, we do not want to have ref links in the
    middle of the final document."""
    # Find all ref link:
    p_nodes = [c for c in md.children if isinstance(c, Paragraph)]
    ref_nodes = [c for c in md.children if isinstance(c, RefLink)]

    # Inline ref links
    for p in p_nodes:

        def repl(match_obj):
            ref = match_obj.group("ref")
            ref_links = (n for n in ref_nodes if n.ref == ref)
            ref_link = next(ref_links, None)
            if not ref_link:
                sys.stderr.write(f"No ref for [{ref}]\n")
                return f"[{ref}]"
            url = ref_link.link.strip()
            new = f"[{ref}]({url})"
            sys.stderr.write(f"Inline `[{ref}]` to `{new}`\n")
            return new

        p.content = re.sub(r"\[(?P<ref>.+?)]", repl, p.content)

    # Delete ref links
    md.remove_nodes(ref_nodes)


def main() -> int:
    # Identify version
    with open("packages/hurl/Cargo.toml", "rb") as f:
        data = tomllib.load(f)
    version = data["package"]["version"]
    version = version.replace("-SNAPSHOT", "")
    sys.stderr.write(f"version:{version}\n")

    standalone_md = MarkdownDoc()

    add_sections(
        doc=standalone_md,
        title="Introduction",
        files=[
            "docs/home.md",
        ],
    )

    add_sections(
        doc=standalone_md,
        title="Getting Started",
        files=[
            "docs/installation.md",
            "docs/manual.md",
            "docs/samples.md",
            "docs/running-tests.md",
            "docs/frequently-asked-questions.md",
        ],
    )

    add_sections(
        doc=standalone_md,
        title="File Format",
        files=[
            "docs/hurl-file.md",
            "docs/entry.md",
            "docs/request.md",
            "docs/response.md",
            "docs/capturing-response.md",
            "docs/asserting-response.md",
            "docs/filters.md",
            "docs/templates.md",
            "docs/grammar.md",
        ],
    )

    add_sections(
        doc=standalone_md,
        title="Resources",
        files=[
            "docs/license.md",
        ],
    )

    # Make the cover
    toc_txt = standalone_md.toc()
    toc = Paragraph(content=toc_txt)
    standalone_md.children.insert(0, toc)

    title = Header(title="Hurl Documentation", level=1)
    standalone_md.children.insert(0, title)
    ws = Whitespace(content="\n")
    standalone_md.children.insert(1, ws)
    date = datetime.today().strftime("%d-%m-%Y")
    title = Header(title=f"Version {version} - {date}", level=2)
    standalone_md.children.insert(2, title)
    ws = Whitespace(content="\n")
    standalone_md.children.insert(3, ws)

    standalone = standalone_md.to_text()
    standalone = rewrite_content(text=standalone, version=version)

    print(standalone)
    return os.EX_OK


def rewrite_content(text: str, version: str) -> str:
    """Some hardcoded replacement."""
    return (
        text.replace("/docs/assets/img/", "https://hurl.dev/assets/img/")
        .replace('<div id="home-demo"></div>', "")
        .replace("[Blog](blog.md)", "[Blog](https://hurl.dev/blog)")
        .replace(
            "[Tutorial](#file-format-tutorial/your-first-hurl-file)",
            "[Tutorial](https://hurl.dev/docs/tutorial/your-first-hurl-file.html)",
        )
        .replace(
            "[Documentation](#getting-started-installation)",
            "[Documentation](https://hurl.dev)",
        )
        .replace(
            f" (download [HTML](/docs/standalone/hurl-{version}.html), [PDF](/docs/standalone/hurl-{version}.pdf), [Markdown](/docs/standalone/hurl-{version}.md))",
            "",
        )
        .replace("/docs/asserting-response.html#", "#file-format-asserting-response-")
        .replace(
            '<a href="/docs/capturing-response.html">',
            '<a href="#file-format-capturing-response-capturing-response">',
        )
        .replace(
            '<a href="#method">Method</a>',
            '<a href="#file-format-request-method">Method</a>',
        )
        .replace('<a href="#url">URL</a>', '<a href="#file-format-request-url">URL</a>')
        .replace(
            '<a href="#headers">HTTP request headers</a>',
            '<a href="#file-format-request-headers">HTTP request headers</a>',
        )
        .replace(
            '<a href="#options">Options</a>',
            '<a href="#file-format-options">Options</a>',
        )
        .replace(
            '<a href="#query-parameters">query strings</a>',
            '<a href="#file-format-request-query-parameters">query strings</a>',
        )
        .replace(
            '<a href="#form-parameters">form params</a>',
            '<a href="#file-format-request-form-parameters">form params</a>',
        )
        .replace(
            '<a href="#cookies">cookies</a>',
            '<a href="#file-format-request-cookies">cookies</a>',
        )
        .replace(
            '<a href="#basic-authentication">authentication</a>',
            '<a href="#file-format-request-basic-authentication">authentication</a>',
        )
        .replace(
            '<a href="#body">HTTP request body</a>',
            '<a href="#file-format-request-body">HTTP request body</a>',
        )
        .replace(
            "[UUID v4 random string]",
            "[UUID v4 random string](https://en.wikipedia.org/wiki/Universally_unique_identifier)",
        )
        .replace(
            "[RFC 3339]",
            "[RFC 3339](https://www.rfc-editor.org/rfc/rfc3339)",
        )
    )


if __name__ == "__main__":
    main()
