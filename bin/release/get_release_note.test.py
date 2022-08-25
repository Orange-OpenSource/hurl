#!/usr/bin/env python3
import unittest

from get_release_note import *


ISSUES = [
    Issue(number=1, tags=["enhancement"], author="bob", pulls=[Pull("url1", "pull1")]),
    Issue(
        number=2,
        tags=["bug"],
        author="bill",
        pulls=[Pull("url2", "pull2"), Pull("url3", "pull3")],
    ),
    Issue(
        number=3,
        tags=["ignore", "enhancement"],
        author="bob",
        pulls=[Pull("url4", "pull4")],
    ),
    Issue(number=4, tags=["enhancement"], author="bob", pulls=[Pull("url4", "pull4")]),
]

PULLS = [
    Pull("url1", "pull1", ["enhancement"], [1]),
    Pull("url2", "pull2", ["bug"], [2]),
    Pull("url3", "pull3", ["bug"], [2]),
    Pull("url4", "pull4", ["ignore", "enhancement"], [3, 4]),
]


class GetReleaseNoteTest(unittest.TestCase):
    def test_authors_from_issues(self):
        self.assertEqual(["bob", "bill"], authors_from_issues(ISSUES))

    def test_pulls_from_issues(self):
        self.assertEqual(PULLS, pulls_from_issues(ISSUES))

    def test_generate_md(self):
        self.assertEqual(
            """[1.0.0 (2022-01-01)](https://github.com/Orange-OpenSource/hurl/blob/master/CHANGELOG.md#1.0.0)
========================================================================================================================

Thanks to
[@bob](https://github.com/bob),
[@bill](https://github.com/bill),


Enhancements:

 * pull1 [#1](https://github.com/Orange-OpenSource/hurl/issues/1)

 * pull4 [#3](https://github.com/Orange-OpenSource/hurl/issues/3) [#4](https://github.com/Orange-OpenSource/hurl/issues/4)


Bugs Fixed:

 * pull2 [#2](https://github.com/Orange-OpenSource/hurl/issues/2)

 * pull3 [#2](https://github.com/Orange-OpenSource/hurl/issues/2)
""",
            generate_md(
                milestone="1.0.0",
                date=datetime.datetime(2022, 1, 1),
                pulls=PULLS,
                authors=["bob", "bill"],
            ),
        )


if __name__ == "__main__":
    unittest.main()
