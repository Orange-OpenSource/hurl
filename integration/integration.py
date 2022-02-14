#!/usr/bin/env python3
import glob
import test_echo
import test_lint
import test_format
import test_hurl


def get_files(glob_expr):
    return sorted([f.replace("\\", "/") for f in glob.glob(glob_expr)])


def main():
    # Static run (without server)
    [
        test_echo.test(f)
        for f in get_files("tests_ok/*.hurl")
        + get_files("tests_failed/*.hurl")
        + get_files("tests_error_lint/*.hurl")
    ]
    [test_format.test("json", f) for f in get_files("tests_ok/*.hurl")]
    [test_format.test("json", f) for f in get_files("tests_failed/*.hurl")]
    [test_format.test("html", f) for f in get_files("tests_ok/*.hurl")]
    [test_format.test("html", f) for f in get_files("tests_failed/*.hurl")]
    [test_lint.test(f) for f in get_files("tests_error_lint/*.hurl")]
    [test_hurl.test(f) for f in get_files("tests_error_parser/*.hurl")]

    # Dynamic run (with server)
    [
        test_hurl.test(f)
        for f in get_files("tests_ok/*.hurl")
        + get_files("tests_failed/*.hurl")
        + get_files("ssl/*.hurl")
    ]

    print("test integration ok!")


if __name__ == "__main__":
    main()
