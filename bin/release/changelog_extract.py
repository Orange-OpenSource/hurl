#!/usr/bin/env python3
# bin/release/changelog_extract.py 1.8.0
import sys


def extract(changelog_file, version):
    print_line = False
    for line in open(changelog_file).readlines():
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
        print("  bin/release/changelog_extract.py 1.8.0")
        sys.exit(1)

    version = sys.argv[1]
    extract("CHANGELOG.md", version)


if __name__ == "__main__":
    main()
