#!/usr/bin/env python3
import glob
import re
import test_lint
import test_format
import test_hurl
import test_script
import platform


def get_files(glob_expr):
    return sorted([f.replace("\\", "/") for f in glob.glob(glob_expr)])


def accept(f: str) -> bool:
    """Returns True if file `f` should be run, False otherwise."""
    return not re.match(r".*\.\d+\.hurl$", f)


def main():
    # Static run (without server)
    [test_format.test("json", f) for f in get_files("tests_ok/*.hurl")]
    [test_format.test("json", f) for f in get_files("tests_failed/*.hurl")]
    [test_format.test("html", f) for f in get_files("tests_ok/*.hurl")]
    [test_format.test("html", f) for f in get_files("tests_failed/*.hurl")]
    [test_lint.test(f) for f in get_files("tests_error_lint/*.hurl")]
    [test_hurl.test(f) for f in get_files("tests_error_parser/*.hurl")]

    # Dynamic run (with server)
    [
        test_hurl.test(f)
        for f in get_files("tests_ok/*.hurl") + get_files("tests_failed/*.hurl")
        if accept(f)
    ]
    # Run test scripts
    extension = "ps1" if platform.system() == "Windows" else "sh"
    script_files = (
        get_files("tests_ok/*." + extension)
        + get_files("tests_failed/*." + extension)
        + get_files("ssl/*." + extension)
    )
    for f in sorted(script_files):
        test_script.test(f)

    print("test integration ok!")


if __name__ == "__main__":
    main()
