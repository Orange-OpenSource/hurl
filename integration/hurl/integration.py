#!/usr/bin/env python3
import glob
import platform
import re
import sys

sys.path.append("..")
import test_script


def get_files(glob_expr):
    return sorted([f.replace("\\", "/") for f in glob.glob(glob_expr)])


def accept(f: str) -> bool:
    """Returns True if file `f` should be run, False otherwise."""
    return not re.match(r".*\.\d+\.hurl$", f)


def main():
    # Run test scripts
    extension = "ps1" if platform.system() == "Windows" else "sh"
    script_files = (
        get_files("tests_ok/*." + extension)
        + get_files("tests_ok_not_linted/*." + extension)
        + get_files("tests_failed/*." + extension)
        + get_files("tests_failed_not_linted/*." + extension)
        + get_files("tests_error_parser/*." + extension)
        + get_files("tests_ssl/*." + extension)
    )
    for f in sorted(script_files):
        test_script.test(f)

    print("Test integration hurl ok!")


if __name__ == "__main__":
    main()
