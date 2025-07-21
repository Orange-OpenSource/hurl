#!/usr/bin/env python3
import argparse
import datetime
import json
import os
import sys

import requests


def get_latest_release(token: str | None) -> None | tuple[str, datetime]:
    """Returns the latest Rust release available."""
    url = "https://api.github.com/repos/rust-lang/rust/releases"
    headers = {}
    if token:
        headers["Authorization"] = f"Bearer {token}"
    r = requests.get(url, headers=headers)
    if r.status_code != 200:
        sys.stderr.write(f"Error GET {url} {r.status_code}\n")
        sys.stderr.write(f"{r.text}\n")
        return None

    releases = json.loads(r.text)
    latest_release = releases[0]
    version = latest_release["tag_name"]
    date_str = latest_release["published_at"]
    date = datetime.datetime.strptime(date_str, "%Y-%m-%dT%H:%M:%SZ")
    return version, date


def get_current_version() -> str:
    """Returns the current Rust version used by the project."""
    return os.popen("cargo --version").read().split(" ")[1]


def main():
    parser = argparse.ArgumentParser(
        description="Check if Hurl uses the latest Rust version"
    )
    parser.add_argument(
        "num_days_before_error",
        type=int,
        metavar="NUM_DAYS_BEFORE_ERROR",
        help="Interval in days before raising an error if Hurl is not using latest Rust",
    )
    parser.add_argument("--token", help="GitHub authentication token")
    args = parser.parse_args()

    num_days_before_error = args.num_days_before_error
    token = args.token

    ret = get_latest_release(token=token)
    if not ret:
        sys.exit(2)

    latest_version, date = ret
    current_version = get_current_version()
    if current_version < latest_version:
        sys.stderr.write(
            f"Rust version must be updated from {current_version} to the latest version {latest_version}\n"
        )
        days_before_now = datetime.datetime.now() - date
        if days_before_now > datetime.timedelta(days=num_days_before_error):
            sys.exit(1)
    else:
        sys.stderr.write(f"Latest Rust version: {latest_version}\n")
        sys.stderr.write(f"Hurl Rust version:   {current_version}\n")


if __name__ == "__main__":
    main()
