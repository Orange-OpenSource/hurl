#!/usr/bin/env python3
import sys
import re


def header():
   return '''---
layout: doc
title: Man Page
---
# {{ page.title }}
'''


def escape(s):
    return s.replace('<', '&lt;').replace('--', '\\-\\-')


def add_anchor_for_h2(s):
    lines = []
    p = re.compile('^## (.*)$')
    for line in s.split('\n'):
        m = p.match(line)
        if m:
            value = m.group(1)
            anchor = value.lower().strip().replace(' ', '-')
            lines.append('## ' + value + ' {#' + anchor + '}')
        else:
            lines.append(line)
    return '\n'.join(lines)


def main():
    input_file = sys.argv[1]
    lines = open(input_file).readlines()
    s = ''.join(lines)
    s = escape(s)
    s = add_anchor_for_h2(s)
    print(header() + s)


if __name__ == '__main__':
    main()
