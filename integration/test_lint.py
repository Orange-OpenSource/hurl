#!/usr/bin/env python3
# lint hurl file
#
import sys
import subprocess

def test(hurl_file):
    cmd = ['hurlfmt', '--check', hurl_file]
    print(' '.join(cmd))
    result = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if result.returncode != 1:
        print(f'return code => expected: 1  actual {result.returncode}')
        sys.exit(1)

    err_file = hurl_file.replace('.hurl','.err')
    expected = open(err_file).read().strip()
    actual = result.stderr.decode("utf-8").strip() 
    if actual != expected:
        print('>>> error in stderr')
        print(f'actual: <{actual}>\nexpected: <{expected}>')
        sys.exit(1)

    cmd = ['hurlfmt', hurl_file]
    print(' '.join(cmd))
    result = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    err_file = hurl_file.replace('.hurl','.hurl.lint')
    expected = open(err_file).read().strip()
    actual = result.stdout.decode("utf-8").strip() 
    if actual != expected:
        print('>>> error in stdout')
        print(f'actual: <{actual}>\nexpected: <{expected}>')
        sys.exit(1)


def main():
    for hurl_file in sys.argv[1:]:
        test(hurl_file)


if __name__ == '__main__':
    main()



