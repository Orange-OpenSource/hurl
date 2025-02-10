#!/usr/bin/env python3
import glob
import platform
import re
import sys

sys.path.append("..")
import test_format
import test_script


def get_files(glob_expr):
    return sorted([f.replace("\\", "/") for f in glob.glob(glob_expr)])


def accept(f: str) -> bool:
    """Returns True if file `f` should be run, False otherwise."""
    return not re.match(r".*\.\d+\.hurl$", f)


def main():
    hurl_files = get_files("tests_export/*.hurl")
    [test_format.test("hurl", f) for f in hurl_files]
    [test_format.test("json", f) for f in hurl_files]
    [test_format.test("html", f) for f in hurl_files]

    extension = "ps1" if platform.system() == "Windows" else "sh"
    script_files = get_files("tests_ok/*." + extension) + get_files(
        "tests_failed/*." + extension
    )
    for f in sorted(script_files):
        test_script.test(f)

    print("Test integration hurlfmt ok!")


if __name__ == "__main__":
    main()
