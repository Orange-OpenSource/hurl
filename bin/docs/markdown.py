"""Process Markdown document.

This module allows to manipulate Markdown document:
- create a navigable Markdown object from a text,
- add, remove child nodes to a Markdown object,
- extend a Markdown document with another Markdown document,
- provide utility methods to construct toc.
"""
import re
import unicodedata
from textwrap import dedent
from typing import List, Optional

from parser import Parser


class Node:
    """Represent the base class for a Markdown document token."""

    content: Optional[str]

    def __init__(self, content: Optional[str]) -> None:
        self.content = content


class Code(Node):
    """A code block token (https://daringfireball.net/projects/markdown/syntax#precode)."""

    pass


class Paragraph(Node):
    """A paragraph token (https://daringfireball.net/projects/markdown/syntax#p)."""

    pass


class Whitespace(Node):
    """A whitespace token."""

    pass


def build_header(title: str, level: int) -> str:
    """Constructs a header in Markdown format.

    Arg:
        title: title of the header.
        level: 1 base index of the header level
    """
    hashes = "#" * level
    return f"{hashes} {title}\n"


class Header(Node):
    """A header token (https://daringfireball.net/projects/markdown/syntax#header)."""

    title: str
    level: int

    def __init__(self, title: str, level: int) -> None:
        super().__init__(content=None)
        self.title = title
        self.level = level
        self.update_content()

    def indent(self, count: int) -> None:
        """Indent or dedent a header

        Args:
            count: number of level to indent, can be negative to dedent.
        """
        self.level += count
        self.update_content()

    def update_content(self) -> None:
        self.content = build_header(title=self.title, level=self.level)


class RefLink(Node):
    """A reference link token (https://daringfireball.net/projects/markdown/syntax#link)."""

    ref: str
    link: str

    def __init__(self, ref: str, link: str) -> None:
        super().__init__(content=None)
        self.ref = ref
        self.link = link
        self.update_content()

    def update_content(self) -> None:
        self.content = f"[{self.ref}]: {self.link}\n"


def parse_paragraph(parser: Parser) -> Paragraph:
    content = ""
    while parser.peek() != "":
        if parser.peek() == "\n":
            content += parser.read()
            line = parser.peek_while(lambda it: it != "\n")
            if is_blank(line):
                return Paragraph(content=content)
            continue
        content += parser.read()
    return Paragraph(content=content)


def is_blank(line: str) -> bool:
    """Return True if line is made of whitespace, False otherwise."""
    for c in line:
        if not is_whitespace(c):
            return False
    return True


def is_whitespace(c: str) -> bool:
    """Return True if c is a whitespace, False otherwise."""
    return c == " " or c == "\t" or c == "\n"


def parse_whitespace(parser: Parser) -> Whitespace:
    """Parse and return a whitespace token."""
    content = parser.read_while(is_whitespace)
    return Whitespace(content=content)


def parse_code(parser: Parser) -> Code:
    """Parse and return a code block token."""
    separator = parser.read(3)
    content = separator

    while parser.peek() != "":
        c = parser.peek(3)
        if c == separator:
            content += parser.read(3)
            return Code(content=content)
        content += parser.read()
    return Code(content=content)


def parse_header(parser: Parser) -> Header:
    """Parse and return a header token."""
    hashes = parser.read_while(lambda it: it == "#")
    _ = parser.read_while(lambda it: is_whitespace(it))
    title = parser.read_while(lambda it: it != "\n")
    _ = parser.read()
    return Header(title=title, level=len(hashes))


def parse_ref_link(parser: Parser) -> RefLink:
    """Parse and return a reference link token."""
    line = parser.read_while(lambda it: it != "\n")
    _ = parser.read()
    ret = re.match(r"\[(?P<ref>.+)]:\s+(?P<link>.+)", line)
    assert ret is not None
    return RefLink(ref=ret.group("ref"), link=ret.group("link"))


def parse_markdown(text: str) -> "MarkdownDoc":
    """Parse a Markdown text and return a document instance."""
    processed_text = text
    parser = Parser(buffer=processed_text)

    root = MarkdownDoc()

    while parser.peek() != "":
        node: Node
        c = parser.peek()

        # Whitespace parsing:
        if is_whitespace(c):
            node = parse_whitespace(parser=parser)
            root.add_child(node)
            continue

        # Code parsing:
        if c == "-" or c == "~" or c == "`":
            sep = parser.peek(3)
            if sep == "---" or sep == "~~~" or sep == "```":
                node = parse_code(parser=parser)
                root.add_child(node)
                continue

        # Header parsing:
        if c == "#":
            node = parse_header(parser=parser)
            root.add_child(node)
            continue

        # Parse Reference-style Links
        if c == "[":
            line = parser.peek_while(lambda it: it != "\n")
            if re.match(r"\[.+]: .+", line):
                node = parse_ref_link(parser=parser)
                root.add_child(node)
                continue

        # Default node parsing:
        node = parse_paragraph(parser=parser)
        root.add_child(node)

    return root


class MarkdownDoc:
    """A class used to represent Markdown document.

    Attributes:
        children: children nodes of this document.
    """

    children: List[Node]

    def __init__(self) -> None:
        self.children = []

    def add_child(self, node) -> None:
        """Add a node to the document."""
        self.children.append(node)

    def find_first(self, func, start: Optional[Node] = None) -> Optional[Node]:
        """Search the first child node that meet a criteria.

        Args:
            func: a callable predicate to filter against.
            start: a node to start the search from (it can be the returned result).
        """
        if start:
            start_index = self.children.index(start)
        else:
            start_index = 0
        for child in self.children[start_index:]:
            if func(child):
                return child
        return None

    def to_text(self) -> str:
        """Return the text representation of this document."""
        ref_links_nodes = [c for c in self.children if isinstance(c, RefLink)]
        other_nodes = [c for c in self.children if not isinstance(c, RefLink)]
        nodes = [*other_nodes, *ref_links_nodes]
        return "".join([node.content for node in nodes if node.content])

    def indent(self, count: int = 1) -> None:
        """Indent all headers of a specified count level."""
        for c in self.children:
            if isinstance(c, Header):
                c.indent(count=count)

    def extend(self, other: "MarkdownDoc") -> None:
        """Extend the current document with another Markdown document instance."""
        self.children.extend(other.children)

    def insert_node(self, start: Node, node: Node) -> None:
        """Insert a child node to the current document, after a specified node."""
        index = self.children.index(start)
        self.children.insert(index, node)

    def insert_nodes(self, start: Node, nodes: List[Node]) -> None:
        """Insert children nodes to the current document, after a specified node."""
        index = self.children.index(start)
        self.children[index:index] = nodes

    def remove_node(self, node: Node) -> None:
        """Remove a child node."""
        try:
            index = self.children.index(node)
            self.children.pop(index)
        except ValueError:
            pass

    def remove_nodes(self, nodes: List[Node]) -> None:
        """Remove children nodes."""
        self.children = [node for node in self.children if node not in nodes]

    def slice(self, node_a: Node, node_b: Node) -> List[Node]:
        """Return a slice of the current children nodes

        Args:
            node_a: lower node (included in the returned slice)
            node_b: upper node (excluded from the returned slice)
        """
        index_a = self.children.index(node_a)
        index_b = self.children.index(node_b)
        return self.children[index_a:index_b]

    def next_node(self, node: Node) -> Optional[Node]:
        """Return the following node of a specified child node."""
        index = self.children.index(node)
        if index < len(self.children):
            return self.children[index + 1]
        else:
            return None

    def previous_node(self, node: Node) -> Optional[Node]:
        """Return the following node of a specified child node."""
        index = self.children.index(node)
        if index > 0:
            return self.children[index - 1]
        else:
            return None

    def toc(self) -> str:
        """Return a table-of-content of the current document."""

        def slugify(value: str) -> str:
            value = (
                unicodedata.normalize("NFKD", value)
                .encode("ascii", "ignore")
                .decode("ascii")
            )
            value = re.sub(r"[^\w\s/-]", "", value).strip().lower()
            return re.sub(r"[-\s]+", "-", value).replace("/", "")

        headers = [child for child in self.children if isinstance(child, Header)]
        toc = dedent(
            """\
        Table of Contents
        =================
        """
        )
        for header in headers:
            indent = "   " * header.level
            slug = slugify(header.title)
            line = f"{indent}* [{header.title}](#{slug})\n"
            toc += line
        return toc
