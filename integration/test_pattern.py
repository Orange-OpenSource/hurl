#!/usr/bin/env python3
import unittest

from test_script import escape_regex_metacharacters, parse_pattern


class PatternTest(unittest.TestCase):
    def test_no_escaping(self):
        self.assertEqual("^Hello World!$", parse_pattern("Hello World!"))

    def test_regex(self):
        self.assertEqual("^Hello .*!$", parse_pattern("Hello <<<.*>>>!"))

    def test_json(self):
        self.assertEqual("""^{"time":\d+}$""", parse_pattern("""{"time":<<<\d+>>>}"""))

    def test_escape_regex_metacharacters(self):
        self.assertEqual("""\\*\\*\\*""", escape_regex_metacharacters("***"))
        self.assertEqual("""\\\\""", escape_regex_metacharacters("\\"))


if __name__ == "__main__":
    unittest.main()
