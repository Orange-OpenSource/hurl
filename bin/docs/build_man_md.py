#!/usr/bin/env python3
"""Build Grammar Markdown Man File.

This script converts Hurl man file to Markdown suitable for the Hurl canonical docs.

This tool takes the Hurl man file as a first argument.

Examples:
    $ python3 build_man_md.py ../../docs/man/hurl.md > ../../docs/man-page.md

"""
import re
import sys
from pathlib import Path
from typing import List

from markdown import (
    parse_markdown,
    MarkdownDoc,
    Header,
    Paragraph,
    Whitespace,
    Node,
)


def normalize_h2(doc: MarkdownDoc) -> None:
    h2s = [h for h in doc.children if isinstance(h, Header) and h.level == 2]
    for h2 in h2s:
        # Add exception for www acronym
        if h2.title == "WWW":
            continue
        h2.title = h2.title.title()
        h2.update_content()


def process_table(doc: MarkdownDoc, nodes: List[Node], col_name: str) -> None:
    """Transform the list of items from the source man document to a beautiful HTML tables.

    This can be used to transform options, variables and environment sections.
    """

    def escape(s):
        return s.replace("<", "&lt;").replace(">", "&gt;")

    new_nodes = [
        Whitespace(content="\n"),
        Paragraph(content=f"{col_name} | Description\n --- | --- \n"),
    ]

    h3s = [n for n in nodes if isinstance(n, Header)]
    for h3 in h3s:
        name_raw = h3.title

        # Try to match name and anchor
        r = re.compile(r"""(.+) \{#(.+)}""")
        m = r.match(name_raw)
        if m:
            _id = m.group(2)
            text = escape(m.group(1))
            name = f'<a href="#{_id}" id="{_id}"><code>{text}</code></a>'
        else:
            name = f"`{name_raw}`"

        next_h = doc.find_first(
            lambda it: isinstance(it, Header), start=doc.next_node(h3)
        )
        first_p = doc.find_first(
            lambda it: isinstance(it, Paragraph), start=doc.next_node(h3)
        )
        assert first_p is not None
        last_p = next_h
        while last_p and not isinstance(last_p, Paragraph):
            last_p = doc.previous_node(last_p)
        assert last_p is not None
        next_node = doc.next_node(last_p)
        assert next_node is not None
        paragraphs = doc.slice(first_p, next_node)
        paragraphs_contents = [p.content for p in paragraphs if p.content]
        description = "".join(paragraphs_contents)
        description = description.replace("\n", "<br/>")

        new_node = Paragraph(content=f"{name} | {description}\n")
        new_nodes.append(new_node)

    # Delete all previous options:
    previous_node = doc.previous_node(nodes[0])
    assert previous_node is not None
    doc.insert_nodes(start=previous_node, nodes=new_nodes)
    doc.remove_nodes(nodes)


def main():
    input_file = sys.argv[1]
    src = Path(input_file).read_text()

    man = parse_markdown(text=src)

    normalize_h2(man)

    # Transform all h3 options, environment var and exit code to tables
    options_h2 = man.find_first(
        lambda it: isinstance(it, Header) and it.title == "Options"
    )
    environment_h2 = man.find_first(
        lambda it: isinstance(it, Header) and it.title == "Environment"
    )
    exit_codes_h2 = man.find_first(
        lambda it: isinstance(it, Header) and it.title == "Exit Codes"
    )
    www_h2 = man.find_first(lambda it: isinstance(it, Header) and it.title == "WWW")

    first_option_h3 = man.find_first(
        lambda it: isinstance(it, Header) and it.level == 3, start=options_h2
    )
    options = man.slice(first_option_h3, environment_h2)
    process_table(doc=man, nodes=options, col_name="Option")

    first_env_h3 = man.find_first(
        lambda it: isinstance(it, Header) and it.level == 3, start=environment_h2
    )
    envs = man.slice(first_env_h3, exit_codes_h2)
    process_table(doc=man, nodes=envs, col_name="Variable")

    first_exit_h3 = man.find_first(
        lambda it: isinstance(it, Header) and it.level == 3, start=exit_codes_h2
    )
    exits = man.slice(first_exit_h3, www_h2)
    process_table(doc=man, nodes=exits, col_name="Value")

    print("# Man Page\n\n" + man.to_text())


if __name__ == "__main__":
    main()
