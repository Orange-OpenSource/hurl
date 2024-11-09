#!/usr/bin/env python3
"""Check Rust direct and transitive dependencies licenses.

This script checks that there is no dependencies with unauthorized license (GPL like).

Examples:
    $ python3 bin/check/license.py
"""

import json
import subprocess
from typing import List, Tuple


def main():
    deps = get_deps()
    check_licenses(deps)


def is_authorized(name: str) -> bool:
    for licence in [
        "MIT",
        "Apache-2.0",
        "Zlib",
        "CC0-1.0",
        "MPL-2.0",
        "BSD-2-Clause",
        "BSD-3-Clause",
        "Unicode-3.0",
    ]:
        if licence in name:
            return True
    return False


def is_forbidden(name: str) -> bool:
    for licence in ["GPL"]:
        if licence in name:
            return True
    return False


def check_licenses(deps: List[Tuple[str, str, str, str]]):
    authorized = []
    forbidden = []
    unknown = []
    for dep in deps:
        lic = dep[3]
        if is_authorized(lic):
            authorized.append(dep)
        elif is_forbidden(lic):
            forbidden.append(dep)
        else:
            unknown.append(dep)
    print("Authorized:")
    for name, repository, version, lic in authorized:
        name_str = f"\x1b[1;34m{name}\x1b[0m"
        lic_str = f"\x1b[1;32m{lic}\x1b[0m"
        print(f"  {name_str} {version} {repository}: {lic_str}")

    print("Forbidden:")
    for name, repository, version, lic in forbidden:
        name_str = f"\x1b[1;34m{name}\x1b[0m"
        lic_str = f"\x1b[1;31m{lic}\x1b[0m"
        print(f"  {name_str} {version} {repository}: {lic_str}")

    print("Unknown:")
    for name, repository, version, lic in unknown:
        name_str = f"\x1b[1;34m{name}\x1b[0m"
        lic_str = f"\x1b[1;33m{lic}\x1b[0m"
        print(f"  {name_str} {version} {repository}: {lic_str}")

    if len(forbidden) > 0:
        print("There are forbidden licenses")
        exit(1)

    if len(unknown) > 0:
        print("There are unknown licenses")
        exit(2)


def get_deps() -> List[Tuple[str, str, str, str]]:
    """Returns a list of crates name and licenses"""
    p = subprocess.run(
        [
            "cargo",
            "metadata",
            "--format-version",
            "1",
        ],
        capture_output=True,
        text=True,
    )
    if p.returncode != 0:
        print("Error calling cargo metadata")
        exit(1)
    data = json.loads(p.stdout)
    packages = data["packages"]
    licenses = [
        (p["name"], p["repository"], p["version"], p["license"]) for p in packages
    ]
    return licenses


if __name__ == "__main__":
    main()
