#!/usr/bin/env python3
# echo hurl file
# The file is parsed and output exactly as the input
#
import codecs
import os
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


def test(format_type, hurl_file):
    output_file = hurl_file.replace(".hurl", "." + format_type)
    if not os.path.exists(output_file):
        return
    cmd = ["hurlfmt", "--format", format_type, hurl_file]
    print(" ".join(cmd))
    result = subprocess.run(cmd, stdout=subprocess.PIPE)
    expected = open(output_file, encoding="utf-8").read().strip()
    actual = decode_string(result.stdout)
    if actual != expected:
        print(f">>> error in stdout for {format_type}")
        print(f"actual: <{actual}>\nexpected: <{expected}>")
        sys.exit(1)


def main():
    if len(sys.argv) < 2:
        print("usage: test_format.py json|html HURL_FILE..")
        sys.exit(1)
    format_type = sys.argv[1]

    for hurl_file in sys.argv[2:]:
        test(format_type, hurl_file)


if __name__ == "__main__":
    main()
