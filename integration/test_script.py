#!/usr/bin/env python3
# Test script file.
#
import codecs
import sys
import subprocess
import os
import argparse
from typing import Optional


def decode_string(encoded: bytes) -> str:
    """Decodes bytes to string from inferring encoding."""
    if encoded.startswith(codecs.BOM_UTF8):
        return encoded.decode("utf-8-sig")
    elif encoded.startswith(codecs.BOM_UTF16):
        encoded = encoded[len(codecs.BOM_UTF16) :]
        return encoded.decode("utf-16")
    else:
        # No BOM to determine encoding, try utf-8
        return encoded.decode("utf-8")


def test(script_file: str):
    """Runs a script, exit the process if there is an error.

    Arguments:
    script_file -- the script file to run
    """
    cmd = (
        "pwsh " + script_file
        if script_file.endswith("ps1")
        else script_file
    )
    # subprocess.run(['pwsh ' + script_file], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    print(cmd)
    result = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)

    # exit code
    expected = get_expected_exit_code(script_file)
    if result.returncode != expected:
        print(">>> error in return code")
        print(f"expected: {expected}  actual:{result.returncode}")
        stderr = decode_string(result.stderr).strip()
        if stderr != "":
            print(stderr)
        stdout = decode_string(result.stdout).strip()
        if stdout != "":
            sys.setdefaultencoding("utf-8")
            sys.stdout.write(stdout + "\n")

        sys.exit(1)

    # stdout
    expected = get_expected_stdout(script_file)
    actual = result.stdout
    if actual != expected:
        print(">>> error in stdout")
        print(f"actual: <{actual}>\nexpected: <{expected}>")
        sys.exit(1)


def get_expected_exit_code(script_file: str) -> int:
    """Runs expected exit code for a given test script"""

    f = os.path.splitext(script_file)[0] + ".exit"
    if os.path.exists(f):
        expected = int(open(f, encoding="utf-8").read().strip())
    else:
        expected = 0
    return expected


def get_expected_stdout(script_file: str) -> Optional[str]:
    """Runs expected stdout for a given test script"""

    f = os.path.splitext(script_file)[0] + ".out"
    if os.path.exists(f):
        value = open(f, "rb").read()
    else:
        value = None
    return value


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("file", type=str, nargs="+", metavar="FILE")
    args = parser.parse_args()
    for script_file in args.file:
        test(script_file=script_file)


if __name__ == "__main__":
    main()
