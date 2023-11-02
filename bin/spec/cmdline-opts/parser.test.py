#!/usr/bin/env python3
import unittest

from option import Option, parse_key_value


class OptionParserTest(unittest.TestCase):
    def test_parse_key_value(self):
        self.assertTrue(parse_key_value("xx") is None)
        self.assertEqual(parse_key_value("a: b"), ("a", "b"))

    def test_parse_connect_timeout(self):
        option = Option.parse(
            """name: connect_timeout
long: connect-timeout
value: SECONDS
value_default: 300
value_parser: u64
help: Maximum time allowed for connection
---
Maximum time in seconds that you allow Hurl’s connection to take.

See also -m, --max-time.
"""
        )
        print(option)
        self.assertEqual(
            Option(
                name="connect_timeout",
                long="connect-timeout",
                short=None,
                value="SECONDS",
                value_default="300",
                value_parser="u64",
                help="Maximum time allowed for connection",
                conflict=None,
                append=False,
                deprecated=False,
                description="Maximum time in seconds that you allow Hurl’s connection to take.\n\nSee also -m, --max-time.",
            ),
            option,
        )

    def test_parse_connect_to(self):
        option = Option.parse(
            """name: connect_to
long: connect-to
value: HOST1:PORT1:HOST2:PORT2
help: For a request to the given HOST1:PORT1 pair, connect to HOST2:PORT2 instead
multi: append
---
For a request to the given HOST1:PORT1 pair, connect to HOST2:PORT2 instead.
"""
        )
        print(option)
        self.assertEqual(
            Option(
                name="connect_to",
                long="connect-to",
                short=None,
                value="HOST1:PORT1:HOST2:PORT2",
                value_default=None,
                value_parser=None,
                help="For a request to the given HOST1:PORT1 pair, connect to HOST2:PORT2 instead",
                conflict=None,
                append=True,
                deprecated=False,
                description="For a request to the given HOST1:PORT1 pair, connect to HOST2:PORT2 instead.",
            ),
            option,
        )

    def test_fail_at_end(self):
        option = Option.parse(
            """name: fail_at_end
long: fail-at-end
help: Fail at end
deprecated: true
---

    """
        )
        print(option)
        self.assertEqual(
            Option(
                name="fail_at_end",
                long="fail-at-end",
                short=None,
                value=None,
                value_parser=None,
                value_default=None,
                help="Fail at end",
                conflict=None,
                append=False,
                deprecated=True,
                description="",
            ),
            option,
        )


if __name__ == "__main__":
    unittest.main()
