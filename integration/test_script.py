#!/usr/bin/env python3
# Test script file.
#
import codecs
import sys
import subprocess
import os
import argparse
import re


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
        "pwsh -Command " + script_file + " ; exit $LASTEXITCODE"
        if script_file.endswith("ps1")
        else script_file
    )
    print(cmd)
    result = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)

    test_exit_code(os.path.splitext(script_file)[0] + ".exit", result)
    test_stdout(os.path.splitext(script_file)[0] + ".out", result)
    test_stdout_pattern(os.path.splitext(script_file)[0] + ".out.pattern", result)
    test_stderr(os.path.splitext(script_file)[0] + ".err", result)
    test_stderr_pattern(os.path.splitext(script_file)[0] + ".err.pattern", result)


def test_exit_code(f, result) -> int:
    """test exit code"""
    if os.path.exists(f):
        expected = int(open(f, encoding="utf-8").read().strip())
    else:
        expected = 0
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

    return expected


def test_stdout(f, result):
    """test stdout"""

    if not os.path.exists(f):
        return

    expected = open(f, "rb").read()
    actual = result.stdout
    if actual != expected:
        print(">>> error in stdout")
        print(f"actual: <{actual}>\nexpected: <{expected}>")
        sys.exit(1)


def test_stdout_pattern(f, result):
    """test stdout with pattern lines"""
    if not os.path.exists(f):
        return

    expected = open(f, encoding="utf-8").read()

    # curl debug logs are too dependent on the context, so we filter
    # them and not take them into account for testing differences.
    expected = remove_curl_debug_lines(expected)
    expected_lines = expected.split("\n")
    expected_pattern_lines = [parse_pattern(line) for line in expected_lines]

    actual = decode_string(result.stdout)
    actual = remove_curl_debug_lines(actual)
    actual_lines = re.split(r"\r?\n", actual)

    if len(actual_lines) != len(expected_pattern_lines):
        print(">>> error in stout / mismatch in number of lines")
        print(
            f"actual: {len(actual_lines)} lines\nexpected: {len(expected_pattern_lines)} lines"
        )
        sys.exit(1)
    for i in range(len(expected_pattern_lines)):
        if not re.match(expected_pattern_lines[i], actual_lines[i]):
            print(f">>> error in stout in line {i+1}")
            print(f"actual: <{actual_lines[i]}>")
            print(
                f"expected: <{expected_lines[i]}> (translated to regex <{expected_pattern_lines[i]}>)"
            )
            sys.exit(1)


def test_stderr(f, result):
    """test stderr"""

    if not os.path.exists(f):
        return

    expected = open(f, encoding="utf-8").read()
    actual = decode_string(result.stderr)
    if actual != expected:
        print(">>> error in stderr")
        print(f"actual  : <{actual}>\nexpected: <{expected}>")
        sys.exit(1)


def test_stderr_pattern(f, result):
    """test stderr with pattern lines"""

    if not os.path.exists(f):
        return

    expected = open(f, encoding="utf-8").read()

    # curl debug logs are too dependent on the context, so we filter
    # them and not take them into account for testing differences.
    expected = remove_curl_debug_lines(expected)
    expected_lines = expected.split("\n")
    expected_pattern_lines = [parse_pattern(line) for line in expected_lines]

    actual = decode_string(result.stderr)
    actual = remove_curl_debug_lines(actual)
    actual_lines = re.split(r"\r?\n", actual)

    if len(actual_lines) != len(expected_pattern_lines):
        print(">>> error in stderr / mismatch in number of lines")
        print(
            f"actual: {len(actual_lines)} lines\nexpected: {len(expected_pattern_lines)} lines"
        )
        sys.exit(1)
    for i in range(len(expected_pattern_lines)):
        if not re.match(expected_pattern_lines[i], actual_lines[i]):
            print(f">>> error in stderr in line {i+1}")
            print(f"actual: <{actual_lines[i]}>")
            print(
                f"expected: <{expected_lines[i]}> (translated to regex <{expected_pattern_lines[i]}>)"
            )
            sys.exit(1)


def parse_pattern(s: str) -> str:
    """Transform a stderr pattern to a regex"""
    # Escape regex metacharacters
    for c in ["\\", ".", "(", ")", "[", "]", "^", "$", "*", "+", "?", "|"]:
        s = s.replace(c, "\\" + c)

    s = re.sub("~+", ".*", s)
    s = "^" + s + "$"
    return s


def remove_curl_debug_lines(text: str) -> str:
    """Removes curl debug logs from text and returns the new text."""
    lines = text.split("\n")
    lines = [line for line in lines if not line.startswith("**")]
    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("file", type=str, nargs="+", metavar="FILE")
    args = parser.parse_args()
    for script_file in args.file:
        test(script_file=script_file)


if __name__ == "__main__":
    main()
