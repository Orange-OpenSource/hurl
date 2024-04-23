#!/usr/bin/env python3
"""Create Release note from GitHub Issues and Pull Requests for a given version

Example:
    $ python3 bin/release/get_release_note.py 1.7.0

"""
import argparse
import datetime
import json
import sys
from typing import List, Optional

import requests
from bs4 import BeautifulSoup

hurl_repo_url = "https://github.com/Orange-OpenSource/hurl"


class Pull:
    def __init__(
        self,
        url: str,
        description: str,
        tags: Optional[List[str]] = None,
        issues: Optional[List[int]] = None,
    ):
        if tags is None:
            tags = []
        if issues is None:
            issues = []
        self.url = url
        self.description = description
        self.tags = tags
        self.issues = issues

    def __repr__(self):
        return 'Pull("%s", "%s","%s", %s)' % (
            self.url,
            self.description,
            str(self.tags),
            str(self.issues),
        )

    def __eq__(self, other):
        """Overrides the default implementation"""
        if isinstance(other, Pull):
            if self.url != other.url:
                return False
            if self.description != other.description:
                return False
            if self.tags != other.tags:
                return False
            if self.issues != other.issues:
                return False
            return True
        return False


class Issue:
    def __init__(self, number: int, tags: List[str], author: str, pulls: List[Pull]):
        self.number = number
        self.tags = tags
        self.author = author
        self.pulls = pulls

    def __repr__(self):
        return (
            'Issue(\n    number=%s,\n    tag=["%s"],\n    author="%s",\n    pulls=[%s]\n)'
            % (
                self.number,
                ",".join(['"%s"' % t for t in self.tags]),
                self.author,
                ",".join([str(p) for p in self.pulls]),
            )
        )


def release_note(milestone: str, token: Optional[str]) -> str:
    """return markdown release note for the given milestone"""
    date = datetime.datetime.now()
    milestone_number = get_milestone(title=milestone, token=token)
    issues = get_issues(milestone_number=milestone_number, token=token)
    pulls = pulls_from_issues(issues)
    authors = [
        author
        for author in authors_from_issues(issues)
        if author not in ["jcamiel", "lepapareil", "fabricereix"]
    ]
    return generate_md(milestone, date, pulls, authors)


def pulls_from_issues(issues: List[Issue]) -> List[Pull]:
    """return list of pulls from list of issues"""
    pulls: dict[str, Pull] = {}
    for issue in issues:
        for pull in issue.pulls:
            if pull.url in pulls:
                saved_pull = pulls[pull.url]
                for tag in issue.tags:
                    if tag not in saved_pull.tags:
                        saved_pull.tags.append(tag)
                saved_pull.issues.append(issue.number)
            else:
                if pull.url.startswith("/Orange-OpenSource/hurl"):
                    pull.tags = issue.tags
                    pull.issues.append(issue.number)
                    pulls[pull.url] = pull

    return list(pulls.values())


def get_issues(milestone_number: int, token: Optional[str]) -> List[Issue]:
    """Return issues for the given milestone and tags"""
    path = "/issues?milestone=%s&state=closed&per_page=100" % milestone_number
    response = github_get(path=path, token=token)
    issues = []
    for issue_json in json.loads(response):
        if "pull_request" in issue_json:
            continue
        number = issue_json["number"]
        tags = []
        if "labels" in issue_json:
            labels = issue_json["labels"]
            tags = [label["name"] for label in labels]
        author = issue_json["user"]["login"]
        pulls = get_linked_pulls(issue_number=number, token=token)
        issue = Issue(number, tags, author, pulls)
        issues.append(issue)
    return issues


def get_linked_pulls(issue_number: int, token: Optional[str]) -> List[Pull]:
    """return linked pull request for a given issue"""
    # Webscapping the webpage issue
    # because the API does not provide the relationship between issues and Pull request

    url = "https://github.com/Orange-OpenSource/hurl/issues/%d" % issue_number
    sys.stderr.write("* GET %s\n" % url)
    headers = {}
    if token:
        headers["Authorization"] = f"Bearer {token}"
    r = requests.get(url, headers=headers)
    html = r.text
    pulls = webscrapping_linked_pulls(html)
    return pulls


def webscrapping_linked_pulls(html: str) -> List[Pull]:
    soup = BeautifulSoup(html, "html.parser")
    links = soup.select("development-menu a")
    pulls = []
    for link in links:
        url = link["href"]
        if not isinstance(url, str):
            continue
        if url == "/Orange-OpenSource/hurl":
            continue
        description = "".join(link.getText()).strip()
        pull = Pull(url, description)
        pulls.append(pull)
    return pulls


def authors_from_issues(issues: List[Issue]) -> List[str]:
    """return list of unique authors from a list of issues"""
    authors = []
    for issue in issues:
        author = issue.author
        if author not in authors:
            authors.append(author)
    return authors


def generate_md(
    milestone: str, date: datetime.datetime, pulls: List[Pull], authors: List[str]
) -> str:
    """Generate Markdown"""

    s = "[%s (%s)](%s)" % (
        milestone,
        date.strftime("%Y-%m-%d"),
        hurl_repo_url + "/blob/master/CHANGELOG.md#" + milestone,
    )
    s += "\n========================================================================================================================"
    s += "\n\nThanks to"
    for author in authors:
        s += "\n[@%s](https://github.com/%s)," % (author, author)

    categories = {
        "breaking": "Breaking Changes",
        "enhancement": "Enhancements",
        "bug": "Bugs Fixed",
        "security": "Security Issues Fixed",
    }

    for category in categories:
        category_pulls = [pull for pull in pulls if category in pull.tags]
        if len(category_pulls) > 0:
            s += "\n\n" + categories[category] + ":" + "\n\n"
        for pull in category_pulls:
            issues = " ".join(
                "[#%s](%s/issues/%s)" % (issue, hurl_repo_url, issue)
                for issue in pull.issues
            )
            s += "* %s %s\n" % (pull.description, issues)

    s += "\n"
    return s


def get_milestone(title: str, token: Optional[str]) -> int:
    """Return milestone number"""
    path = "/milestones?state=all"
    response = github_get(path=path, token=token)
    for milestone in json.loads(response):
        if milestone["title"] == title:
            return milestone["number"]
    return -1


def github_get(path: str, token: Optional[str]) -> str:
    """Execute an HTTP GET with request"""
    github_api_url = "https://api.github.com/repos/Orange-OpenSource/hurl"
    url = github_api_url + path
    sys.stderr.write("* GET %s\n" % url)
    headers = {}
    if token:
        headers["Authorization"] = f"Bearer {token}"
    r = requests.get(url, headers=headers)
    if r.status_code != 200:
        raise Exception("HTTP Error %s - %s" % (r.status_code, r.text))
    return r.text


def main():
    parser = argparse.ArgumentParser(
        description="Get Hurl release notes from issues/PR"
    )
    parser.add_argument("version", help="Hurl release version ex 4.2.0")
    parser.add_argument("--token", help="GitHub authentication token")
    args = parser.parse_args()
    if args.version == "":
        raise Exception("version can not be empty")
    print(release_note(milestone=args.version, token=args.token))


if __name__ == "__main__":
    main()
