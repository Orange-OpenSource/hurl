#!/usr/bin/env python3
# cat CHANGELOG.md | bin/release/changelog_extract.py 1.8.0
import sys


def extract(version):
    print_line = False
    for line in sys.stdin.readlines():
        if "CHANGELOG" in line and line.startswith("["):
            if line[1:].startswith(version):
                print_line = True
            else:
                print_line = False
        if print_line:
            print(line.rstrip())


def main():
    if len(sys.argv) < 2:
        print("usage:")
        print("  cat CHANGELOG.md | bin/release/changelog_extract.py 1.8.0")
        sys.exit(1)

    version = sys.argv[1]
    extract(version)


if __name__ == "__main__":
    main()
