#!/usr/bin/env python3
# lint hurl file
#
import codecs
import sys
import subprocess


def decode_string(encoded):
    if encoded.startswith(codecs.BOM_UTF8):
        return encoded.decode("utf-8-sig")
    elif encoded.startswith(codecs.BOM_UTF16):
        encoded = encoded[len(codecs.BOM_UTF16) :]
        return encoded.decode("utf-16")
    else:
        return encoded.decode()


def test(hurl_file):
    cmd = ["hurlfmt", "--check", hurl_file]
    print(" ".join(cmd))
    result = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if result.returncode != 1:
        print(f"return code => expected: 1  actual {result.returncode}")
        sys.exit(1)

    err_file = hurl_file.replace(".hurl", ".err")
    expected = open(err_file).read().strip()
    actual = decode_string(result.stderr).strip()
    if actual != expected:
        print(">>> error in stderr")
        print(f"actual: <{actual}>\nexpected: <{expected}>")
        sys.exit(1)

    cmd = ["hurlfmt", hurl_file]
    print(" ".join(cmd))
    result = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    err_file = hurl_file.replace(".hurl", ".hurl.lint")
    expected = open(err_file).read().strip()
    actual = decode_string(result.stdout).strip()
    if actual != expected:
        print(">>> error in stdout")
        print(f"actual: <{actual}>\nexpected: <{expected}>")
        sys.exit(1)


def main():
    for hurl_file in sys.argv[1:]:
        test(hurl_file)


if __name__ == "__main__":
    main()
