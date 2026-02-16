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
    is_windows = platform.system() == "Windows"
    extension = "ps1" if is_windows else "sh"
    script_files = (
        get_files("tests_ok/**/*." + extension)
        + get_files("tests_ok_not_linted/*." + extension)
        + get_files("tests_failed/**/*." + extension)
        + get_files("tests_failed_not_linted/*." + extension)
        + get_files("tests_error_parser/*." + extension)
        + get_files("tests_ssl/*." + extension)
    )
    for f in sorted(script_files):
        test_script.test(script_file=f, use_tty=True)

    # Some tests need a "terminal" env, contrary to previous tests where standard output and error output are captured.
    # These tests use a PTY only available on *Nix platform (Windows WSL included, but not "standard" Windows).
    if not is_windows:
        script_files = get_files("tests_pty/**/*.sh")
        for f in sorted(script_files):
            test_script.test(script_file=f, use_tty=False)

    print("Test integration hurl ok!")


if __name__ == "__main__":
    main()
