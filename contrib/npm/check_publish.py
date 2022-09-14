#!/usr/bin/env python3
"""
Examples:
    $ python3 contrib/npm/check_archive.py 1.6.1
"""
import sys
import json
from pathlib import Path
import hashlib
from urllib import request


def bold(text: str) -> str:
    return f"\x1b[1m{text}\x1b[0m"


def bold_blue(text: str) -> str:
    return f"\x1b[1;34m{text}\x1b[0m"


def bold_green(text: str) -> str:
    return f"\x1b[1;32m{text}\x1b[0m"


def bold_red(text: str) -> str:
    return f"\x1b[1;31m{text}\x1b[0m"


def check_archive(version: str):
    print(bold_blue("Checking archives:"))
    path = Path("contrib/npm/hurl/platform.json")
    platforms = json.loads(path.read_text())

    for platform in platforms:
        target = platform["rust_target"]
        extension = platform["archive_extension"]
        expected_checksum = platform["checksum"]
        url = f"  https://github.com/Orange-OpenSource/hurl/releases/download/{version}/hurl-{version}-{target}{extension}"
        print(f"  Downloading: {bold(url)}")
        with request.urlopen(url) as response:
            if response.status != 200:
                print(bold_red("  Checksum KO"))
                sys.exit(1)
            body = response.read()

        m = hashlib.sha256()
        m.update(body)
        actual_checksum = m.hexdigest()
        print(f"  Checksum:    {bold(actual_checksum)}")

        if actual_checksum != expected_checksum:
            print(bold_red("  Checksum KO"))
            sys.exit(1)
        else:
            print(bold_green("  Checksum OK"))
        print()


def check_version(version: str):
    print(bold_blue("Checking version:"))
    path = Path("contrib/npm/hurl/package.json")
    package = json.loads(path.read_text())
    expected_version = version
    actual_version = package["version"]
    if actual_version != expected_version:
        print(
            bold_red(
                f"  Version KO actual={actual_version} expected={expected_version}"
            )
        )
        sys.exit(1)
    else:
        print(bold_green("  Version OK"))


def check_manual(version: str):
    print(bold_blue("Checking manual:"))
    print()
    pass


def main(version: str):
    check_version(version)
    check_manual(version)
    check_archive(version)


if __name__ == "__main__":
    main(sys.argv[1])
