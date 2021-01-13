#!/usr/bin/env python3
import sys
import glob 
import test_echo
import test_lint
import test_format
import test_hurl

def main():
    # Static run (without server)
    [test_echo.test(f) for f in glob.glob('tests/*.hurl') + glob.glob('tests_error_lint/*.hurl')]
    [test_format.test('json', f) for f in glob.glob('tests/*.hurl')]
    [test_format.test('html', f) for f in glob.glob('tests/*.hurl')]
    [test_lint.test(f) for f in glob.glob('tests_error_lint/*.hurl')]
    [test_hurl.test(f) for f in glob.glob('tests_error_parser/*.hurl')]

    # Dynamic run (with server)
    [test_hurl.test(f) for f in glob.glob('tests/*.hurl')]

    print('test integration ok!')


if __name__ == '__main__':
    main()


