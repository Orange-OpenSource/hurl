#!/usr/bin/env python3
# echo hurl file
# The file is parsed and output exactly as the input
#
import codecs
import os
import subprocess
import sys
from typing import Literal


def decode_string(encoded: bytes) -> str:
    """Decodes bytes by guessing the encoding and returns a string."""
    if encoded.startswith(codecs.BOM_UTF8):
        return encoded.decode("utf-8-sig")
    elif encoded.startswith(codecs.BOM_UTF16):
        encoded = encoded[len(codecs.BOM_UTF16) :]
        return encoded.decode("utf-16")
    else:
        return encoded.decode()


def test(format_type: Literal["hurl", "html", "json"], hurl_file: str):
    """
    Exports a Hurl file to different format:
    - format_type == "hurl": input file is linted and compared to `foo.lint.hurl`
    - format_type == "html": input file is exported to HTML and compared to `foo.html`
    - format_type == "json": input file is exported to JSON and compared to `foo.json`
    """
    extension = ".lint.hurl" if format_type == "hurl" else ("." + format_type)
    output_file = hurl_file.replace(".hurl", extension)
    if not os.path.exists(output_file):
        return
    cmd = ["hurlfmt", "--out", format_type, hurl_file]
    print(" ".join(cmd))
    result = subprocess.run(cmd, stdout=subprocess.PIPE)
    expected = open(output_file, encoding="utf-8").read()
    actual = decode_string(result.stdout)
    if actual != expected:
        print(f">>> error in stdout for {format_type}")
        print(f"actual: <{actual}>\nexpected: <{expected}>")
        sys.exit(1)


def main():
    if len(sys.argv) < 2:
        print("usage: test_format.py hurl|json|html HURL_FILE..")
        sys.exit(1)
    format_type = sys.argv[1]

    for hurl_file in sys.argv[2:]:
        test(format_type, hurl_file)


if __name__ == "__main__":
    main()
