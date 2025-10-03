#!/usr/bin/env python3
import glob
import platform
import sys

sys.path.append("..")
import test_script


def get_files(glob_expr: str) -> list[str]:
    return sorted([f.replace("\\", "/") for f in glob.glob(glob_expr, recursive=True)])


def main():
    # Run test scripts
    extension = "ps1" if platform.system() == "Windows" else "sh"
    script_files = (
        get_files("tests_error_parser/*." + extension)
    )
    for f in sorted(script_files):
        test_script.test(f)

    print("Test integration hurl ok!")


if __name__ == "__main__":
    main()
