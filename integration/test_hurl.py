#!/usr/bin/env python3
# Test Hurl files.
#
import codecs
import sys
import subprocess
import os
import platform
import check_json_output
import re
import argparse


def decode_string(encoded: bytes) -> str:
    """Decodes bytes to string from infering encoding."""
    if encoded.startswith(codecs.BOM_UTF8):
        return encoded.decode("utf-8-sig")
    elif encoded.startswith(codecs.BOM_UTF16):
        encoded = encoded[len(codecs.BOM_UTF16) :]
        return encoded.decode("utf-16")
    else:
        return encoded.decode()


def get_os() -> str:
    """Returns `linux-fedora`, `linux`, `osx` or `windows`
    can add more specific linux variant if needed
    """
    if platform.system() == "Linux":
        if os.path.exists("/etc/fedora-release"):
            return "linux-fedora"
        return "linux"
    elif platform.system() == "Darwin":
        return "osx"
    elif platform.system() == "Windows":
        return "windows"
    else:
        raise Error("Invalid Platform " + platform.system())


def test(hurl_file: str) -> int:
    """Runs a Hurl file and returns the exit code.

    Arguments:
    hurl_file -- the Hurl file to run
    use_cargo -- true to run hurl with 'cargo run', else just run 'hurl"
    """
    options_file = hurl_file.replace(".hurl", ".options")

    # For .curl file, we can have specific os expected file in order to test
    # os differences like included path (/ vs \ path components separator)
    os_curl_file = hurl_file.replace(".hurl", "." + get_os() + ".curl")
    if os.path.exists(os_curl_file):
        curl_file = os_curl_file
    else:
        curl_file = hurl_file.replace(".hurl", ".curl")

    json_output_file = hurl_file.replace(".hurl", ".output.json")
    profile_file = hurl_file.replace(".hurl", ".profile")

    options = []
    if os.path.exists(options_file):
        options = open(options_file).read().strip().split("\n")
    if os.path.exists(curl_file):
        options.append("--verbose")

    if os.path.exists(json_output_file):
        options.append("--json")

    env = os.environ.copy()
    if os.path.exists(profile_file):
        for line in open(profile_file).readlines():
            line = line.strip()
            if line == "":
                continue
            index = line.index("=")
            name = line[:index]
            value = line[(index + 1) :]
            env[name] = value

    cmd = ["hurl", hurl_file] + options
    print(" ".join(cmd))
    result = subprocess.run(
        cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, env=env
    )

    # exit code
    f = hurl_file.replace(".hurl", ".exit")
    expected = int(open(f).read().strip())
    if result.returncode != expected:
        print(">>> error in return code")
        print(f"expected: {expected}  actual:{result.returncode}")
        stderr = decode_string(result.stderr).strip()
        if stderr != "":
            print(stderr)
        return 1

    # stdout
    f = hurl_file.replace(".hurl", ".out")
    if os.path.exists(f):
        expected = open(f, "rb").read()
        actual = result.stdout
        if actual != expected:
            print(">>> error in stdout")
            print(f"actual: <{actual}>\nexpected: <{expected}>")
            return 1

    # stdout with textual pattern / line per line
    f = hurl_file.replace(".hurl", ".out.pattern")
    if os.path.exists(f):
        expected = open(f, "r").read()
        actual = decode_string(result.stdout)
        expected_lines = expected.split("\n")
        expected_pattern_lines = [parse_pattern(line) for line in expected_lines]
        actual_lines = actual.split("\n")
        if len(actual_lines) != len(expected_pattern_lines):
            print(">>> error in stderr / mismatch in number of lines")
            print(
                "actual: %d lines\nexpected: %d lines"
                % (len(actual_lines), len(expected_lines))
            )
            return 1
        for i in range(len(expected_pattern_lines)):
            if not re.match(expected_pattern_lines[i], actual_lines[i]):
                print(f">>> error in stdout in line {i+1}")
                print(f"actual: <{actual_lines[i]}>")
                print(
                    f"expected: <{expected_lines[i]}> (translated to regex <{expected_pattern_lines[i]}>)"
                )
                return 1

    # stdout (json)
    if os.path.exists(json_output_file):
        expected = open(json_output_file).read()
        actual = result.stdout
        check_json_output.check(expected, actual)

    # stderr
    f = hurl_file.replace(".hurl", "." + get_os() + ".err")
    if os.path.exists(f):
        expected = open(f).read().strip()
        actual = decode_string(result.stderr).strip()
        if actual != expected:
            print(">>> error in stderr")
            print(f"actual: <{actual}>\nexpected: <{expected}>")
            sys.exit(1)
    else:
        f = hurl_file.replace(".hurl", ".err")
        if os.path.exists(f):
            expected = open(f).read().strip()
            actual = decode_string(result.stderr).strip()
            if expected != actual:
                print(">>> error in stderr")
                print(f"actual: <{actual}>\nexpected: <{expected}>")
                return 1

    # stderr with textual pattern / line per line
    f = hurl_file.replace(".hurl", ".err.pattern")
    if os.path.exists(f):
        expected = open(f, "r").read()
        actual = decode_string(result.stderr)
        expected_lines = expected.split("\n")
        expected_pattern_lines = [parse_pattern(line) for line in expected_lines]
        actual_lines = re.split(r"\r?\n", actual)
        if len(actual_lines) != len(expected_pattern_lines):
            print(">>> error in stderr / mismatch in number of lines")
            print(
                "actual: %d lines\nexpected: %d lines"
                % (len(actual_lines), len(expected_lines))
            )
            return 1
        for i in range(len(expected_pattern_lines)):
            if not re.match(expected_pattern_lines[i], actual_lines[i]):
                print(f">>> error in stderr in line {i+1}")
                print(f"actual: <{actual_lines[i]}>")
                print(
                    f"expected: <{expected_lines[i]}> (translated to regex <{expected_pattern_lines[i]}>)"
                )
                return 1

    # curl output
    if os.path.exists(curl_file):
        expected_commands = []
        for line in open(curl_file, "r").readlines():
            line = line.strip()
            if line == "" or line.startswith("#"):
                continue
            expected_commands.append(line)

        actual = decode_string(result.stderr).strip()
        actual_commands = [
            line[2:] for line in actual.split("\n") if line.startswith("* curl")
        ]

        if len(actual_commands) != len(expected_commands):
            print("curl commands error at %s" % (curl_file))
            print("expected: %d commands" % len(expected_commands))
            print("actual:   %d commands" % len(actual_commands))
            return 1

        for i in range(len(expected_commands)):
            if actual_commands[i] != expected_commands[i]:
                print("curl command error at %s:%i" % (curl_file, i + 1))
                print("expected: %s" % expected_commands[i])
                print("actual:   %s" % actual_commands[i])
                return 1


def parse_pattern(s):
    # Escape regex metacharacters
    for c in [".", "(", ")", "[", "]", "^", "$", "*", "+", "?"]:
        s = s.replace(c, "\\" + c)

    s = re.sub("~+", ".*", s)
    s = "^" + s + "$"
    return s


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("file", type=str, nargs="+", metavar="FILE")
    args = parser.parse_args()
    for hurl_file in args.file:
        ret = test(hurl_file=hurl_file)
        if ret != 0:
            sys.exit(ret)


if __name__ == "__main__":
    main()
