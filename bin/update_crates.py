#!/usr/bin/env python3
import argparse
import datetime as dt
import re
import subprocess
import textwrap
from dataclasses import dataclass
from datetime import datetime
from enum import StrEnum
from pathlib import Path

import requests as req
import tomllib


class Color(StrEnum):
    """Helper enum to print ANSI escape codes."""

    GREY = "\033[0;37m"
    RED = "\033[0;31m"
    GREEN = "\033[0;32m"
    YELLOW = "\033[1;33m"
    BLUE = "\033[0;34m"
    RESET = "\033[0m"


class Crate:
    """A Rust crate: a compilation unit"""

    name: str
    version: str
    repository: str | None
    owner_repo: str | None
    updated_at: datetime | None

    def __init__(self, name: str, version: str):
        self.name = name
        self.version = version
        self.repository = None
        self.owner_repo = None
        self.updated_at = None

    def get_last_version(self) -> str | None:
        """Returns the last version for this crate."""
        ret = subprocess.run(
            ["cargo", "info", self.name], check=True, capture_output=True, text=True
        )
        match = re.search(
            r"^version: ([0-9]+\.[0-9]+\.[0-9]+).*", ret.stdout, re.MULTILINE
        )
        if not match:
            return None
        last_version = match.group(1)
        return last_version

    def fetch_info(self) -> bool:
        """Update crate's repository and owner repo from <crates.io>"""
        self.repository = None
        self.owner_repo = None
        self.updated_at = None
        crate_url = f"https://crates.io/api/v1/crates/{self.name}"
        crate_object = req.get(url=crate_url)
        if crate_object.status_code != 200:
            return False
        crate = crate_object.json()["crate"]
        self.repository = crate["repository"]
        self.updated_at = dt.datetime.fromisoformat(crate["updated_at"])
        if self.repository and self.repository.startswith("https://github.com/"):
            self.owner_repo = self.repository.removeprefix("https://github.com/")
        return True

    def get_release_body(self, version: str, token: str) -> str | None:
        """Get the release body for a given version"""
        headers = {"Accept": "application/vnd.github+json"}
        if token:
            headers["Authorization"] = f"Bearer {token}"
        tags = req.get(
            url=f"https://api.github.com/repos/{self.owner_repo}/git/refs/tags",
            headers=headers,
        )
        if tags.status_code != 200:
            return None
        tag_version = next((tag for tag in tags.json() if version in tag["ref"]), None)
        if not tag_version:
            return None
        tag_name = tag_version["ref"].removeprefix("refs/tags/")
        release = req.get(
            url=f"https://api.github.com/repos/{self.owner_repo}/releases/tags/{tag_name}",
            headers=headers,
        )
        if release.status_code != 200:
            return None
        return release.json().get("body", None)


@dataclass
class CrateUpdate:
    """Represents a crate update."""

    crate: Crate
    old_version: str
    new_version: str


@dataclass
class CargoLockUpdate:
    """Represents a update to a `Cargo.lock` file"""

    added: list[Crate]
    removed: list[Crate]
    updated: list[CrateUpdate]


class CargoLock:
    """Represents a `Cargo.lock` file"""

    lock_file: Path
    packages: list[Crate]

    def __init__(self, lock_file: Path):
        self.lock_file = lock_file
        self.packages = []
        with open(lock_file, "rb") as f:
            data = tomllib.load(f)
            for package in data["package"]:
                package = Crate(name=package["name"], version=package["version"])
                self.packages.append(package)

    def update(self) -> CargoLockUpdate:
        """Updates this lock file and return a `CargoLockUpdate` instance"""
        subprocess.run(["cargo", "update"], check=True, capture_output=True)
        updated_lock = CargoLock(lock_file=self.lock_file)
        current_pkgs = {c.name: c for c in self.packages}
        updated_pkgs = {c.name: c for c in updated_lock.packages}

        # Computes the package diffs
        added = []
        updated = []
        removed = []
        for name, crate in updated_pkgs.items():
            if crate.name not in current_pkgs:
                added.append(crate)
            elif crate.version != current_pkgs[crate.name].version:
                update = CrateUpdate(
                    crate=crate,
                    old_version=current_pkgs[crate.name].version,
                    new_version=crate.version,
                )
                updated.append(update)
        for name, crate in current_pkgs.items():
            if name not in updated_pkgs:
                removed.append(crate)

        # Update the self instance
        self.packages = updated_lock.packages
        return CargoLockUpdate(added=added, removed=removed, updated=updated)


class LocalCrate:
    """A local Rust crate"""

    toml_file: Path
    dependencies: list[Crate]

    def __init__(self, toml_file: Path):
        self.toml_file = toml_file
        self.dependencies = []
        with open(self.toml_file, "rb") as f:
            data = tomllib.load(f)
            for dependency, version in data["dependencies"].items():
                if isinstance(version, dict):
                    # Ignore local dependencies
                    if version.get("path"):
                        continue
                    version = version["version"]
                self.dependencies.append(Crate(name=dependency, version=version))

    def update_dependency(self, crate: Crate, actual_version: str, last_version: str):
        """Update the dependency of this crate in its toml file."""
        toml = self.toml_file.read_text()
        name = re.escape(crate.name)
        toml = re.sub(
            rf'^{name}.*=.*{{.*version.*=.*"{actual_version}"',
            f'{name} = {{ version = "{last_version}"',
            toml,
            count=0,
            flags=re.MULTILINE,
        )
        toml = re.sub(
            rf'^{name}.*=.*"{actual_version}"',
            f'{name} = "{last_version}"',
            toml,
            count=0,
            flags=re.MULTILINE,
        )
        self.toml_file.write_text(toml)

    def __str__(self):
        return f"LocalCrate toml_file={self.toml_file}"


def print_release_note(crate: Crate, version: str, token: str):
    """Prints a crate release, for a given version"""
    note = f"<{crate.repository}>\n\n"
    release_body = crate.get_release_body(version=version, token=token)
    if release_body:
        note += f"~~~\n{release_body}\n~~~\n"
    note = textwrap.indent(note, "    ")
    print(f"\n{note}")


def update_local_crates(
    local_crates: list[LocalCrate], cooldown_days: int, token: str | None
):
    """Updates a list of local crates and returns the number of update crates"""
    now = datetime.now(dt.timezone.utc)

    # Update Cargo.toml
    for local_crate in local_crates:
        print("\n--------------------------------------------------------")
        print(f"### Crates updates for *{local_crate.toml_file}*\n")

        for dep in local_crate.dependencies:
            actual_version = dep.version
            last_version = dep.get_last_version()
            if last_version == actual_version:
                print(f"- {dep.name} {actual_version} {Color.GREEN}newest{Color.RESET}")
                continue

            ret = dep.fetch_info()
            if not ret:
                print(
                    f"- {dep.name} {actual_version} {Color.RED}error fetching info{Color.RESET}"
                )
                continue

            # Check cooldown
            if abs(now - dep.updated_at) < dt.timedelta(days=cooldown_days):
                print(
                    f"- {dep.name} {actual_version} {Color.YELLOW}update {last_version} too recent{Color.RESET}"
                )
                continue

            local_crate.update_dependency(
                crate=dep, actual_version=actual_version, last_version=last_version
            )
            print(
                f"- {dep.name} {actual_version} {Color.BLUE}updated to {last_version}{Color.RESET}"
            )
            print_release_note(crate=dep, version=last_version, token=token)

    # Update Cargo.lock
    lock = CargoLock(lock_file=Path("Cargo.lock"))
    updated_lock = lock.update()

    print("\n--------------------------------------------------------")
    print("### Crates updated for *Cargo.lock*\n")
    for update in updated_lock.updated:
        crate = update.crate
        actual_version = update.old_version
        last_version = update.new_version
        crate.fetch_info()
        print(
            f"- {crate.name} {actual_version} {Color.BLUE}updated to {last_version}{Color.RESET}"
        )
        print_release_note(crate=crate, version=last_version, token=token)

    print("\n--------------------------------------------------------")
    print("### Crates added for *Cargo.lock*\n")
    for crate in updated_lock.added:
        crate.fetch_info()
        print(f"- {crate.name} {crate.version} {Color.BLUE}added{Color.RESET}")
        print_release_note(crate=crate, version=crate.version, token=token)

    print("\n--------------------------------------------------------")
    print("### Crates removed for *Cargo.lock*\n")
    for crate in updated_lock.removed:
        crate.fetch_info()
        print(f"- {crate.name} {crate.version} {Color.GREY}removed{Color.RESET}")
        print_release_note(crate=crate, version=crate.version, token=token)


def main():
    parser = argparse.ArgumentParser(
        description="Update Rust crate to the latest version"
    )
    parser.add_argument("crates", nargs="+", help="crate local path to update")
    parser.add_argument("--token", help="GitHub authentication token")
    parser.add_argument("--cooldown", type=int, default=4)
    args = parser.parse_args()
    crates = [LocalCrate(toml_file=Path(c) / "Cargo.toml") for c in args.crates]
    update_local_crates(
        local_crates=crates, cooldown_days=args.cooldown, token=args.token
    )


if __name__ == "__main__":
    main()
