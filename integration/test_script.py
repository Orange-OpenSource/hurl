#!/usr/bin/env python3
# Test script file.
#
import argparse
import codecs
import os
import re
import subprocess
import sys
import time


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
    if script_file.endswith("ps1"):
        cmd = ["pwsh", "-Command", script_file, ";", "exit $LASTEXITCODE"]
    else:
        cmd = [script_file]
    start_time = time.time()
    result = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    execution_time = round(time.time() - start_time, 4)
    print(f"{' '.join(cmd)} (Duration: {execution_time} seconds)")

    basename = os.path.splitext(script_file)[0]

    _continue = test_exit_code(f"{basename}.exit", result)
    if not _continue:
        print(f"{cmd} - skipped")
        return
    test_stdout(f"{basename}.out", result)
    test_stdout_pattern(f"{basename}.out.pattern", result)
    test_stderr(f"{basename}.err", result)
    test_stderr_pattern(f"{basename}.err.pattern", result)


def test_exit_code(f: str, result: subprocess.CompletedProcess) -> bool:
    """Test actual exit code `result` against an expected exit code in file `f`"""
    if os.path.exists(f):
        expected = int(open(f, encoding="utf-8").read().strip())
    else:
        expected = 0
    actual = result.returncode
    # Exit code 255 is the signal to skip test.
    if actual == 255:
        return False

    if actual != expected:
        print(">>> error in return code")
        print(f"expected: {expected}  actual:{actual}")
        stderr = decode_string(result.stderr).strip()
        if stderr != "":
            print(stderr)
        stdout = decode_string(result.stdout).strip()
        if stdout != "":
            sys.stdout.write(stdout + "\n")

        sys.exit(1)

    return True


def test_stdout(f, result):
    """test stdout"""

    if not os.path.exists(f):
        return

    expected = open(f, "rb").read()
    actual = result.stdout
    if actual != expected:
        print(">>> error in stdout")
        print(f"actual:   <{actual}>\nexpected: <{expected}>")
        sys.exit(1)


def test_stdout_pattern(f, result):
    """test stdout with pattern lines"""
    if not os.path.exists(f):
        return

    expected = open(f, encoding="utf-8").read()

    # curl debug logs are too dependent on the context, so we filter
    # them and not take them into account for testing differences.
    expected = ignore_lines(expected)
    expected_lines = expected.split("\n")
    expected_pattern_lines = [parse_pattern(line) for line in expected_lines]

    actual = decode_string(result.stdout)
    actual = ignore_lines(actual)
    actual_lines = re.split(r"\r?\n", actual)

    if len(actual_lines) != len(expected_pattern_lines):
        print(">>> error in stdout / mismatch in number of lines")
        print(
            f"actual:   {len(actual_lines)} lines\nexpected: {len(expected_pattern_lines)} lines"
        )
        print(f"actual <{actual}>")
        print("# Actual lines")
        for i, line in enumerate(actual_lines):
            print("%2d: %s" % (i, line))
        print("# Expected lines")
        for i, line in enumerate(expected_lines):
            print("%2d: %s" % (i, line))
        print("# Expected Pattern lines")
        for i, line in enumerate(expected_pattern_lines):
            print("%2d: %s" % (i, line))

        sys.exit(1)
    for i in range(len(expected_pattern_lines)):
        if not re.match(expected_pattern_lines[i], actual_lines[i]):
            print(f">>> error in stdout in line {i+1}")
            print("actual:")
            print(actual_lines[i])
            print("expected (pattern):")
            print(expected_lines[i])
            print("expected (regex):")
            print(expected_pattern_lines[i])
            sys.exit(1)


def test_stderr(f, result):
    """test stderr"""

    if not os.path.exists(f):
        return

    expected = ignore_lines(open(f, encoding="utf-8").read())
    actual = ignore_lines(decode_string(result.stderr))
    if actual != expected:
        print(">>> error in stderr")
        print("actual:")
        print(actual)
        print("expected:")
        print(expected)
        sys.exit(1)


def test_stderr_pattern(f, result):
    """test stderr with pattern lines"""

    if not os.path.exists(f):
        return

    expected = open(f, encoding="utf-8").read()

    # curl debug logs are too dependent on the context, so we filter
    # them and not take them into account for testing differences.
    expected = ignore_lines(expected)
    expected_lines = expected.split("\n")
    expected_pattern_lines = [parse_pattern(line) for line in expected_lines]

    actual = decode_string(result.stderr)
    actual = ignore_lines(actual)
    actual_lines = re.split(r"\r?\n", actual)

    if len(actual_lines) != len(expected_pattern_lines):
        print(">>> error in stderr / mismatch in number of lines")
        print(
            f"actual:   {len(actual_lines)} lines\nexpected: {len(expected_pattern_lines)} lines"
        )
        print("# Actual lines")
        for i, line in enumerate(actual_lines):
            print("%2d: %s" % (i, line))
        print("# Expected lines")
        for i, line in enumerate(expected_lines):
            print("%2d: %s" % (i, line))
        print("# Expected Pattern lines")
        for i, line in enumerate(expected_pattern_lines):
            print("%2d: %s" % (i, line))

        sys.exit(1)
    for i in range(len(expected_pattern_lines)):
        if not re.match(expected_pattern_lines[i], actual_lines[i]):
            print(f">>> error in stderr in line {i+1}")
            print("actual:")
            print(actual_lines[i])
            print("expected (pattern):")
            print(expected_lines[i])
            print("expected (regex):")
            print(expected_pattern_lines[i])
            sys.exit(1)


def escape_regex_metacharacters(s) -> str:
    """escape all regex metacharacters in a string"""
    escaped_s = ""
    for c in s:
        escaped_s += escape_regex_metacharacter(c)
    return escaped_s


def escape_regex_metacharacter(c) -> str:
    """escape regex metacharacter"""
    if c in [
        "\\",
        "/",
        ".",
        "{",
        "}",
        "(",
        ")",
        "[",
        "]",
        "^",
        "$",
        "*",
        "+",
        "?",
        "|",
    ]:
        return "\\" + c
    return c


def parse_pattern(s: str) -> str:
    """Create a standard regex string from the custom pattern file"""

    # Split string into alternating tokens: escaping regex metacharacters and no escaping
    # The first token must always be escaped
    tokens = re.split("<<<([^>]+)>>>", s)
    s = ""
    for i, token in enumerate(tokens):
        if i % 2 == 0:
            s += escape_regex_metacharacters(token)
        else:
            s += token

    s = "^" + s + "$"
    return s


def ignore_lines(text: str) -> str:
    """Removes curl debug logs from text and returns the new text."""
    lines = []
    for line in text.split("\n"):
        if line.startswith("**"):  # curl debug info
            continue
        if "libcurl.so.4: no version information available" in line:
            continue
        lines.append(line)

    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("file", type=str, nargs="+", metavar="FILE")
    args = parser.parse_args()
    for script_file in args.file:
        test(script_file=script_file)


if __name__ == "__main__":
    main()
