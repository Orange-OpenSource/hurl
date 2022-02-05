#!/usr/bin/env python3
# Extract hurl file from html output
import sys
from bs4 import BeautifulSoup
import os
import codecs


def test(html_file):
    print(html_file)
    actual = extract_hurl_content(html_file)

    hurl_file = os.path.splitext(html_file)[0] + ".hurl"
    if not os.path.isfile(hurl_file):
        return
    expected = codecs.open(
        hurl_file, encoding="utf-8-sig"
    ).read()  # Input file can be saved with a BOM
    if actual != expected:
        print(">>> error in html file")
        print(f"actual: <{actual}>\nexpected: <{expected}>")
        sys.exit(1)


def extract_hurl_content(hurl_file):
    s = open(hurl_file).read()
    return BeautifulSoup(s, "lxml").text


def main():
    print("** test html output")
    for html_file in sys.argv[1:]:
        test(html_file)


if __name__ == "__main__":
    main()
