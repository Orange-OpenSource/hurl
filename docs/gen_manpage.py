#!/usr/bin/env python3
import sys
import re


def header(version):
    return '.TH hurl 1 "DATE" "hurl %s" " Hurl Manual"' % (version)


def version():
    s = open('../Cargo.toml', 'r').read()
    p = re.compile('version(.*)"')
    p = re.compile('(.*)', re.MULTILINE)
    m = p.match(s)
    return '0.99'


def process_code_block(s):
    p = re.compile("```(.*?)```" , re.DOTALL )
    return p.sub('\\\\f[C]\\1\\\\f[R]', s)


def convert_md(s):

     p = re.compile('^###\s+(.*)')
     s = p.sub('.IP "\\1"', s)

     p = re.compile('^##')
     s = p.sub('.SH', s)


     p = re.compile('\*\*(.*)\*\*\s+')
     s = p.sub('.B \\1\n', s)


     # Remove link Text
     p = re.compile('\[(.*)\]\(.*\)')
     s = p.sub('\\\\fI\\1\\\\fP', s)

     # Remove local anchor
     p = re.compile('{#.*}')
     s = p.sub('', s)
     return s


def main():
    input_file = sys.argv[1]
    data = open(input_file).readlines()
    print(header(version()))

    s = ''.join([convert_md(line) for line in data])
    #s = process_code_block(s)
    print(s)


if __name__ == '__main__':
    main()
