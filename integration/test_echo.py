#!/usr/bin/env python3
# echo hurl file
# The file is parsed and output exactly as the input
#
import sys
import subprocess
import codecs


def decode_string(encoded):
    if encoded.startswith(codecs.BOM_UTF8):
        return encoded.decode("utf-8-sig")
    elif encoded.startswith(codecs.BOM_UTF16):
        encoded = encoded[len(codecs.BOM_UTF16) :]
        return encoded.decode("utf-16")
    else:
        return encoded.decode()


def test(hurl_file):
    cmd = ["hurlfmt", "--no-format", hurl_file]
    print(" ".join(cmd))
    result = subprocess.run(cmd, stdout=subprocess.PIPE)
    expected = codecs.open(
        hurl_file, encoding="utf-8-sig"
    ).read()  # Input file can be saved with a BOM
    actual = decode_string(result.stdout)
    if actual != expected:
        print(">>> error in stdout")
        print(f"actual: <{actual}>\nexpected: <{expected}>")
        sys.exit(1)


def main():
    for hurl_file in sys.argv[1:]:
        test(hurl_file)


if __name__ == "__main__":
    main()
