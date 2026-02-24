#!/usr/bin/env python3
"""Patternize .out.pattern and .err.pattern files for Hurl integration tests.

Usage:
    patternize.py <directory>
"""

import re
import sys
from pathlib import Path
from typing import List


def patternize_out(files: List[Path]) -> None:
    """Patternize a list of .out.pattern files, replacing dynamic values with Hurl patterns."""
    for p in files:
        txt = p.read_text()

        # Timings in JSON
        txt = re.sub(r'"app_connect":\d+', r'"app_connect":<<<\\d+>>>', txt)
        txt = re.sub(r'"connect":\d+', r'"connect":<<<\\d+>>>', txt)
        txt = re.sub(r'"name_lookup":\d+', r'"name_lookup":<<<\\d+>>>', txt)
        txt = re.sub(r'"pre_transfer":\d+', r'"pre_transfer":<<<\\d+>>>', txt)
        txt = re.sub(r'"start_transfer":\d+', r'"start_transfer":<<<\\d+>>>', txt)
        txt = re.sub(r'"total":\d+', r'"total":<<<\\d+>>>', txt)

        txt = re.sub(r"boundary=[a-zA-Z0-9-]+", "boundary=<<<[a-zA-Z0-9-]+>>>", txt)

        p.write_text(txt)


def patternize_err(files: List[Path]) -> None:
    """Patternize a list of .err.pattern files, replacing dynamic values with Hurl patterns."""
    for p in files:
        txt = p.read_text()

        # Patternize some headers
        txt = re.sub(r"hurl/\d\.\d\.\d(-SNAPSHOT)?", "hurl/<<<.*>>>", txt)
        txt = re.sub(
            r"Werkzeug/\d+\.\d+\.\d+ Python/\d+\.\d+\.\d+",
            "Werkzeug/<<<.*?>>> Python/<<<.*>>>",
            txt,
        )

        txt = re.sub(r"in \d+ ms", r"in <<<\\d+>>> ms", txt)

        # Replace date-like strings
        txt = re.sub(
            r"[a-zA-Z]{3}, \d+ [a-zA-Z]{3} \d+ \d+:\d+:\d+ GMT", "<<<.*>>>", txt
        )
        txt = re.sub(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}.\d{6} UTC", "<<<.*>>>", txt)
        txt = re.sub(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}.\d{9} UTC", "<<<.*>>>", txt)
        txt = re.sub(
            r'"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}.\d{6}Z"', r'"<<<.*>>>"', txt
        )

        # Timings in sdterr
        txt = re.sub(r"app_connect: \d+", r"app_connect: <<<\\d+>>>", txt)
        txt = re.sub(r"connect: \d+", r"connect: <<<\\d+>>>", txt)
        txt = re.sub(r"namelookup: \d+", r"namelookup: <<<\\d+>>>", txt)
        txt = re.sub(r"pre_transfer: \d+", r"pre_transfer: <<<\\d+>>>", txt)
        txt = re.sub(r"start_transfer: \d+", r"start_transfer: <<<\\d+>>>", txt)
        txt = re.sub(r"total: \d+", r"total: <<<\\d+>>>", txt)

        p.write_text(txt)


def main(directory: Path) -> None:
    """Patternize all .out.pattern and .err.pattern files found recursively in directory."""
    out_files = [p for p in directory.rglob("*.out.pattern")]
    err_files = [p for p in directory.rglob("*.err.pattern")]

    patternize_out(files=out_files)
    patternize_err(files=err_files)


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: patternize.py <directory>")
        sys.exit(1)
    main(directory=Path(sys.argv[1]))
