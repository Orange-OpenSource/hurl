#!/usr/bin/env python3
# echo hurl file
# The file is parsed and output exactly as the input
#
import sys
import subprocess

def test(hurl_file):
    cmd = ['hurlfmt', '--no-format', hurl_file]
    print(' '.join(cmd))
    result = subprocess.run(cmd, stdout=subprocess.PIPE)
    expected = open(hurl_file).read()
    actual = result.stdout.decode("utf-8") 
    if actual != expected:
        print('>>> error in stdout')
        print(f'actual: <{actual}>\nexpected: <{expected}>')
        sys.exit(1)
        


def main():
    for hurl_file in sys.argv[1:]:
        test(hurl_file)


if __name__ == '__main__':
    main()

