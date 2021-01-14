#!/usr/bin/env python3
# echo hurl file
# The file is parsed and output exactly as the input
#
import sys
import subprocess

def test(format_type, hurl_file):
    cmd = ['hurlfmt', '--format', format_type, hurl_file]
    print(' '.join(cmd))
    result = subprocess.run(cmd, stdout=subprocess.PIPE)
    json_file = hurl_file.replace('.hurl','.' + format_type)
    expected = open(json_file).read().strip()
    actual = result.stdout.decode("utf-8").strip() 
    if actual != expected:
        print('>>> error in stdout')
        print(f'actual: <{actual}>\nexpected: <{expected}>')
        sys.exit(1)


def main():
    if len(sys.argv) < 2:
        print('usage: test_format.py json|html HURL_FILE..')
        sys.exit(1)
    format_type = sys.argv[1] 

    for hurl_file in sys.argv[2:]:
        test(format_type, hurl_file)


if __name__ == '__main__':
    main()

