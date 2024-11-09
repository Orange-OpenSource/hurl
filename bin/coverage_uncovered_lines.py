#!/usr/bin/env python3
import sys

from bs4 import BeautifulSoup

COVERAGE_DIR = "target/coverage"


def uncovered_lines(src_file):
    html_file = COVERAGE_DIR + "/" + src_file + ".html"
    sys.stderr.write(html_file + "\n")
    html = open(html_file).read()
    soup = BeautifulSoup(html, "html.parser")
    elements = soup.select('div[role="row"]')
    lines = []
    for element in elements:
        line = parse_row(element)
        if line is not None:
            lines.append(line)
    return lines


def parse_row(element):
    uncovered = element.select(".has-background-danger-light")
    if len(uncovered) > 0:
        line_number = element.select("div:first-child")[0]["id"]
        line = uncovered[0].select("pre")[0].text
        return line_number, line
    return None


def main():
    sys.stderr.write("Extracting uncovered lines\n")
    for src_file in sys.argv[1:]:
        lines = uncovered_lines(src_file)
        if len(lines) > 0:
            print(src_file)
            for line_number, line in lines:
                print("%s %s" % (line_number, line))


if __name__ == "__main__":
    main()
