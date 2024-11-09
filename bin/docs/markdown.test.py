#!/usr/bin/env python3
import unittest

from markdown import Table


class UtilsTest(unittest.TestCase):
    def test_normalize_table(self):
        src = """\
| a | b | c|
|---|---|--|
| aaaaa | bbbbb | cccc |
| dd | ee | f |\
"""
        normalized = """\
| a     | b     | c    |
|-------|-------|------|
| aaaaa | bbbbb | cccc |
| dd    | ee    | f    |
"""
        table = Table(content=src)
        table.reformat()

        self.assertEqual(normalized, table.content)


if __name__ == "__main__":
    unittest.main()
