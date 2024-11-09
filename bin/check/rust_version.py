#!/usr/bin/env python3
import datetime
import json
import os
import sys

import requests


def get_latest_release():
    r = requests.get("https://api.github.com/repos/rust-lang/rust/releases")
    releases = json.loads(r.text)
    latest_release = releases[0]
    version = latest_release["tag_name"]
    date_str = latest_release["published_at"]
    date = datetime.datetime.strptime(date_str, "%Y-%m-%dT%H:%M:%SZ")
    return version, date


def get_current_version():
    return os.popen("cargo --version").read().split(" ")[1]


def main():
    if len(sys.argv) < 2:
        print("Usage: rust_version.py NUM_DAYS_BEFORE_ERROR")
        sys.exit(1)
    num_days_before_error = int(sys.argv[1])

    latest_version, date = get_latest_release()
    current_version = get_current_version()
    if current_version < latest_version:
        sys.stderr.write(
            "Rust version must be updated from %s to the latest version %s\n"
            % (current_version, latest_version)
        )
        days_before_now = datetime.datetime.now() - date
        if days_before_now > datetime.timedelta(days=num_days_before_error):
            sys.exit(1)


if __name__ == "__main__":
    main()
