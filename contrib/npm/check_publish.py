#!/usr/bin/env python3
"""
Examples:
    $ python3 contrib/npm/check_archive.py 1.6.1
"""

import hashlib
import json
import sys
from pathlib import Path
from urllib import request


def bold(text: str) -> str:
    return f"\x1b[1m{text}\x1b[0m"


def bold_blue(text: str) -> str:
    return f"\x1b[1;34m{text}\x1b[0m"


def bold_green(text: str) -> str:
    return f"\x1b[1;32m{text}\x1b[0m"


def bold_red(text: str) -> str:
    return f"\x1b[1;31m{text}\x1b[0m"


def check_archive(hurl_version: str, package_version: str):
    print(bold_blue("Checking archives:"))
    path = Path("contrib/npm/hurl/platform.json")
    platforms = json.loads(path.read_text())

    for platform in platforms:
        target = platform["rust_target"]
        extension = platform["archive_extension"]
        expected_checksum = platform["checksum"]
        url = f"https://github.com/Orange-OpenSource/hurl/releases/download/{hurl_version}/hurl-{hurl_version}-{target}{extension}"
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
            print(
                bold_red(
                    f"  Checksum KO, please update {target} checksum in contrib/npm/hurl/platform.json"
                )
            )
            sys.exit(1)
        else:
            print(bold_green("  Checksum OK"))
        print()


def check_version(hurl_version: str, package_version: str):
    print(bold_blue("Checking version:"))
    path = Path("contrib/npm/hurl/package.json")
    package = json.loads(path.read_text())
    expected_hurl_version = hurl_version
    actual_hurl_version = package["hurlBinaryVersion"]
    expected_package_version = package_version
    actual_package_version = package["version"]

    if actual_hurl_version != expected_hurl_version:
        print(
            bold_red(
                f"  Hurl version KO actual={actual_hurl_version} expected={expected_hurl_version}, please update "
                f"hurlBinaryVersion in contrib/npm/hurl/package.json"
            )
        )
        sys.exit(1)
    else:
        print(bold_green("  Hurl version OK"))

    if actual_package_version != expected_package_version:
        print(
            bold_red(
                f"  Package version KO actual={actual_package_version} expected={expected_package_version}, please update "
                f"version in contrib/npm/hurl/package.json"
            )
        )
        sys.exit(1)
    else:
        print(bold_green("  Package version OK"))


def check_manual(hurl_version: str, package_version: str):
    print(bold_blue("Checking manual:"))
    print()
    pass


def main(hurl_version: str, package_version):
    check_version(hurl_version, package_version)
    check_manual(hurl_version, package_version)
    check_archive(hurl_version, package_version)

    print(bold("Everything looks OK!"))


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
