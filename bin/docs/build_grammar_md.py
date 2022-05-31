#!/usr/bin/env python3
"""Build Grammar Markdown Documentation File.

This script converts Hurl spec grammar file to Markdown suitable for the Hurl canonical docs.

This tool takes the Hurl spec grammar as a first argument.

Examples:
    $ python3 build_grammar_md.py ../../docs/grammar/hurl.grammar > ../../docs/grammar.md

"""
from pathlib import Path
from textwrap import dedent
from typing import List, Optional
import re
import sys
from parser import Parser


class Token:
    """Represent the base class for a Hurl Grammar token."""

    value: str

    def __init__(self, value: str) -> None:
        self.value = value

    def to_html(self):
        """Returns an HTML representation of this token."""
        return f"{self.value}"


class BeginGroup(Token):
    """A begin group token."""

    def __init__(self) -> None:
        super().__init__("(")


class EndGroup(Token):
    """An end group token."""

    def __init__(self) -> None:
        super().__init__(")")


class Equal(Token):
    """An equal token."""

    def __init__(self) -> None:
        super().__init__("=")

    def to_html(self) -> str:
        return f'<div class="associate">{self.value}</div>'


class Cardinality(Token):
    """A cardinality token."""

    def __init__(self, value: str) -> None:
        super().__init__(value)


class Or(Token):
    """An or token."""

    def __init__(self) -> None:
        super().__init__("|")


class Comment(Token):
    """A comment token."""

    def __init__(self, value: str) -> None:
        super().__init__(value)


class Terminal(Token):
    """A terminal token."""

    def __init__(self, value: str) -> None:
        super().__init__(value)


class NonTerminal(Token):
    """A non-terminal token."""

    def __init__(self, value: str) -> None:
        super().__init__(value)


class Definition(Token):
    """A definition token."""

    def __init__(self, value: str) -> None:
        super().__init__(value)


class Follow(Token):
    """A follow token."""

    def __init__(self) -> None:
        super().__init__(" ")


class FollowEol(Token):
    """A follow end-of-line token."""

    def __init__(self) -> None:
        super().__init__("\n")


class Rule:
    """Represents a rule of the grammar.

    A rule associate a non-terminal token with a list of tokens.

    Attributes:
        non_terminal: the non-terminal token (left side of the rule)
        tokens: lists of tokens represented by the non-terminal (right side of the rule)
    """

    non_terminal: NonTerminal
    tokens: List[Token]

    def __init__(self, non_terminal: NonTerminal, tokens: List[Token]) -> None:
        self.non_terminal = non_terminal
        self.tokens = tokens


class GrammarParser(Parser):
    """A parser for Hurl grammar."""

    def __init__(self, buffer) -> None:
        super().__init__(buffer)

    def parse_grammar(self) -> List[Rule]:
        """Parse grammar and return a list of rules.

        A rule is an association between a left-side non-terminal token and a right-side list of tokens.
        """
        rules: List[Rule] = []
        while self.left() > 0:
            rule = self.parse_rule()
            if rule:
                rules.append(rule)
        return rules

    def parse_rule(self) -> Optional[Rule]:
        """Parse a grammar rule."""
        # Parse potential comment
        if self.peek() == "#":
            _ = self.parse_comment()
            return None

        # Left side of association:
        non_terminal = self.parse_non_terminal()
        _ = self.parse_whitespaces()
        _ = self.parse_equal()
        _ = self.parse_whitespaces()
        sys.stderr.write(f"  parsing non_terminal: {non_terminal.value}\n")

        # Right side:
        tokens: List[Token] = []
        token: Token
        while self.left() > 0:
            c = self.peek()
            if c == "(":
                token = self.parse_begin_group()
            elif c == ")":
                token = self.parse_end_group()
            elif c == "|":
                token = self.parse_or()
            elif c == "<":
                token = self.parse_definition()
            elif c == "*" or c == "?" or c == "+":
                token = self.parse_cardinality()
            elif c == " " or c == "\n":
                sp = self.parse_whitespaces()
                if "\n\n" in sp:
                    break
                if "\n" in sp:
                    token = FollowEol()
                else:
                    token = Follow()
            elif c == '"':
                token = self.parse_terminal()
                sys.stderr.write(f"  parsing terminal: {token.value}\n")
            else:
                token = self.parse_non_terminal()
            tokens.append(token)

        return Rule(non_terminal=non_terminal, tokens=tokens)

    def parse_definition(self) -> Definition:
        """Parse and return a definition token."""

        def is_not_definition_end(current, prev):
            return not (current == ">" and prev != '"')

        c = self.read()
        assert c == "<"
        name = self.read_while_prev(is_not_definition_end)
        c = self.read()
        assert c == ">"
        return Definition(value=name)

    def parse_begin_group(self) -> BeginGroup:
        """Parse and return a begin group token."""
        c = self.read()
        assert c == "("
        return BeginGroup()

    def parse_end_group(self) -> EndGroup:
        """Parse and return an end group token."""
        c = self.read()
        assert c == ")"
        return EndGroup()

    def parse_comment(self) -> Comment:
        """Parse and return an end group token."""
        value = self.read_while(lambda it: it != "\n")
        _ = self.parse_whitespaces()
        return Comment(value=value)

    def parse_non_terminal(self) -> NonTerminal:
        """Parse and return a non-terminal token."""
        name = self.read_while(lambda it: re.search(r"[a-z\-|\d]", it) is not None)
        return NonTerminal(value=name)

    def parse_terminal(self) -> Terminal:
        """Parse and return a terminal token."""
        c = self.read()
        assert c == '"'
        offset = self.offset
        while self.left() > 0:
            c = self.peek()
            if c == "\\":
                _ = self.read(2)
            elif c == '"' and (self.offset != offset):
                break
            else:
                _ = self.read()
        name = self.buffer[offset : self.offset]
        c = self.read()
        assert c == '"'
        return Terminal(value=name)

    def parse_whitespaces(self) -> str:
        """Parse and return whitespaces as a string."""
        return self.read_while(lambda it: it == " " or it == "\n" or it == "\t")

    def parse_cardinality(self) -> Cardinality:
        """Parse and return a cardinality token."""
        c = self.read()
        assert c == "*" or c == "?" or c == "+"
        return Cardinality(value=c)

    def parse_equal(self) -> Equal:
        """Parse and return an equal token."""
        ret = self.read()
        assert ret == "="
        return Equal()

    def parse_or(self) -> Or:
        """Parse and return an or token."""
        ret = self.read()
        assert ret == "|"
        return Or()


def rule_to_html(rule: Rule) -> str:
    """Convert a rule to an HTML representationn"""
    txt = ""
    count = len(rule.tokens)
    for i in range(count):
        t = rule.tokens[i]
        if isinstance(t, Follow):
            txt += ""
        elif isinstance(t, FollowEol):
            txt += "<br>"
            # Manage the alignment
            next_t = rule.tokens[i + 1]
            if not isinstance(next_t, Or):
                txt += "&nbsp;"
        if isinstance(t, Definition):
            txt += f'<span class="definition">&lt;{t.value}&gt;</span>'
        elif isinstance(t, Terminal):
            txt += f'<span class="terminal">"{t.value}"</span>'
        elif isinstance(t, NonTerminal):
            txt += f'<a href="#{t.value}">{t.value}</a>'
        else:
            txt += t.value

    html = ""
    html += f'<div class="rule">\n'
    html += f'  <div class="non-terminal" id="{rule.non_terminal.value}">{rule.non_terminal.value}&nbsp;</div>\n'
    html += f'  <div class="tokens">=&nbsp;{txt}</div>'
    html += f"</div>\n"

    return html


def main():
    sys.stderr.write("Parsing grammar...\n")

    text = Path(sys.argv[1]).read_text()
    parser = GrammarParser(buffer=text)
    rules = parser.parse_grammar()
    body = "".join([rule_to_html(r) for r in rules])
    md = f"""\
# Grammar

## Definitions

Short description:

- operator &#124; denotes alternative,
- operator * denotes iteration (zero or more),
- operator + denotes iteration (one or more),

## Syntax Grammar

<div class="grammar">
{body}
</div>
"""

    print(dedent(md))


if __name__ == "__main__":
    main()
