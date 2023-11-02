#!/usr/bin/env python3
import sys
from option import Option

"""
Format option file
"""


def main():
    if len(sys.argv) < 2:
        print("usage: format.py OPTION_FILE")
        sys.exit(1)
    for option_file in sys.argv[1:]:
        option = Option.parse_file(option_file)
        open(option_file, "w").write(str(option) + "\n")


if __name__ == "__main__":
    main()
